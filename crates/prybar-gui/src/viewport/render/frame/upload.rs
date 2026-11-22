use wgpu::util::DeviceExt;

use super::color::{floats, lines};
use crate::viewport::mesh::{Bounds, Scene};

/// GPU copies of one scene. The grid buffer is rewritten each frame so
/// its color follows the theme without a rebuild.
pub struct SceneBuffers {
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub indices: u32,
    /// Draw ranges, one material each.
    pub parts: Vec<crate::viewport::mesh::Part>,
    pub bones: wgpu::Buffer,
    pub bone_vertices: u32,
    pub grid: wgpu::Buffer,
    pub grid_points: Vec<[f32; 3]>,
}

/// Overlay bones in the app's rust accent, readable on clay and dark.
const BONE: [f32; 4] = [0.9, 0.38, 0.16, 1.0];

pub fn upload(device: &wgpu::Device, scene: &Scene) -> SceneBuffers {
    let init = |label, contents: &[u8], usage| buffer(device, label, contents, usage);
    let bone_points: Vec<[f32; 3]> = scene.bones.iter().flatten().copied().collect();
    let grid_points = grid(&scene.bounds);
    let vertex = init(
        "vertices",
        &floats(&scene.vertices),
        wgpu::BufferUsages::VERTEX,
    );
    let index_bytes: Vec<u8> = scene.indices.iter().flat_map(|i| i.to_le_bytes()).collect();
    SceneBuffers {
        vertex,
        index: init("indices", &index_bytes, wgpu::BufferUsages::INDEX),
        indices: scene.indices.len() as u32,
        parts: scene.parts.clone(),
        bones: init(
            "bones",
            &lines(&bone_points, BONE),
            wgpu::BufferUsages::VERTEX,
        ),
        bone_vertices: bone_points.len() as u32,
        grid: init(
            "grid",
            &lines(&grid_points, [0.0; 4]),
            wgpu::BufferUsages::VERTEX,
        ),
        grid_points,
    }
}

fn buffer(
    device: &wgpu::Device,
    label: &str,
    contents: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        // wgpu rejects empty buffers; a model without bones still gets
        // one, it is just never drawn.
        contents: if contents.is_empty() {
            &[0; 28]
        } else {
            contents
        },
        usage: usage | wgpu::BufferUsages::COPY_DST,
    })
}

/// A square grid under the model, step rounded to a power of ten so
/// cells read as real units, 10 cells each way from the center.
pub fn grid(b: &Bounds) -> Vec<[f32; 3]> {
    let [cx, cy, _] = b.center();
    let step = 10f32.powf((b.radius() / 4.0).log10().round());
    let (cx, cy) = ((cx / step).round() * step, (cy / step).round() * step);
    let half = step * 10.0;
    let z = b.min[2];
    (-10..=10)
        .flat_map(|i| {
            let o = i as f32 * step;
            [
                [cx + o, cy - half, z],
                [cx + o, cy + half, z],
                [cx - half, cy + o, z],
                [cx + half, cy + o, z],
            ]
        })
        .collect()
}
