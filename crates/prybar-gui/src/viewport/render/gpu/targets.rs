/// sRGB BGRA: GPUI images are BGRA bytes, and the sRGB view keeps the
/// lighting math linear.
pub const COLOR: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
pub const DEPTH: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// 4x MSAA: smooth silhouettes, supported by every backend wgpu targets.
pub const SAMPLES: u32 = 4;

/// Offscreen attachments for one frame size, rebuilt only on resize.
pub struct Targets {
    pub width: u32,
    pub height: u32,
    pub msaa: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub resolve: wgpu::Texture,
    pub resolve_view: wgpu::TextureView,
    pub readback: wgpu::Buffer,
    /// Row pitch of `readback`; copies need 256-byte aligned rows.
    pub row: u32,
}

impl Targets {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let msaa = texture(device, width, height, COLOR, SAMPLES);
        let depth = texture(device, width, height, DEPTH, SAMPLES);
        let resolve = texture(device, width, height, COLOR, 1);
        let row = (width * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
            * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: u64::from(row) * u64::from(height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Self {
            width,
            height,
            msaa: msaa.create_view(&Default::default()),
            depth: depth.create_view(&Default::default()),
            resolve_view: resolve.create_view(&Default::default()),
            resolve,
            readback,
            row,
        }
    }
}

fn texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    samples: u32,
) -> wgpu::Texture {
    let copy = if samples == 1 {
        wgpu::TextureUsages::COPY_SRC
    } else {
        wgpu::TextureUsages::empty()
    };
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | copy,
        view_formats: &[],
    })
}
