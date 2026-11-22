use std::time::Duration;

use crate::viewport::render::gpu::targets::Targets;
use crate::viewport::render::gpu::Gpu;

/// Copies the resolved frame out and waits for it: this runs on the
/// render thread, so blocking here never blocks the UI.
pub fn read(gpu: &Gpu, t: &Targets, mut enc: wgpu::CommandEncoder) -> Result<Vec<u8>, String> {
    copy(&mut enc, t);
    let submission = gpu.queue.submit([enc.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    let slice = t.readback.slice(..);
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    let wait = wgpu::PollType::Wait {
        submission_index: Some(submission),
        timeout: Some(Duration::from_secs(5)),
    };
    gpu.device
        .poll(wait)
        .map_err(|e| format!("GPU stalled: {e}"))?;
    rx.recv_timeout(Duration::from_secs(5))
        .map_err(|_| "GPU frame timed out".to_string())?
        .map_err(|e| format!("GPU frame unreadable: {e}"))?;
    let pixels = tight(&slice, t);
    t.readback.unmap();
    pixels
}

fn copy(enc: &mut wgpu::CommandEncoder, t: &Targets) {
    enc.copy_texture_to_buffer(
        t.resolve.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &t.readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(t.row),
                rows_per_image: Some(t.height),
            },
        },
        wgpu::Extent3d {
            width: t.width,
            height: t.height,
            depth_or_array_layers: 1,
        },
    );
}

/// Drops the row padding the copy needed.
fn tight(slice: &wgpu::BufferSlice, t: &Targets) -> Result<Vec<u8>, String> {
    let view = slice
        .get_mapped_range()
        .map_err(|e| format!("GPU frame unreadable: {e}"))?;
    let width = t.width as usize * 4;
    let mut out = Vec::with_capacity(width * t.height as usize);
    for row in view.chunks_exact(t.row as usize) {
        out.extend_from_slice(&row[..width]);
    }
    Ok(out)
}
