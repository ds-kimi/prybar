pub mod material;
pub mod pipelines;
mod states;
pub mod targets;
mod texels;

use std::sync::{Arc, Mutex};

/// Last GPU error, read after each frame.
pub type Errors = Arc<Mutex<Option<String>>>;

/// Device, queue and the one uniform buffer every pipeline reads.
pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub globals: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind: wgpu::BindGroup,
    /// Line polygon mode is optional on some backends; without it the
    /// wireframe toggle is hidden instead of crashing.
    pub wire: bool,
    pub errors: Errors,
}

/// Bytes of the `Globals` struct in both shaders.
pub const GLOBALS: u64 = 28 * 4;

/// Opens the default GPU without a surface: frames go to a texture and
/// come back as pixels, so the viewport works inside any GPUI window.
pub fn open() -> Result<Gpu, String> {
    let instance = instance();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    }))
    .map_err(|e| format!("No GPU for the 3D preview: {e}"))?;
    let wire = adapter
        .features()
        .contains(wgpu::Features::POLYGON_MODE_LINE);
    let errors = Errors::default();
    let (device, queue) = device(&adapter, wire, errors.clone())?;
    let (globals, layout, bind) = globals(&device);
    Ok(Gpu {
        device,
        queue,
        globals,
        layout,
        bind,
        wire,
        errors,
    })
}

fn instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: wgpu::InstanceFlags::default(),
        backend_options: wgpu::BackendOptions::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        display: None,
    })
}

fn device(
    adapter: &wgpu::Adapter,
    wire: bool,
    errors: Errors,
) -> Result<(wgpu::Device, wgpu::Queue), String> {
    let features = if wire {
        wgpu::Features::POLYGON_MODE_LINE
    } else {
        wgpu::Features::empty()
    };
    let desc = wgpu::DeviceDescriptor {
        label: Some("viewport"),
        required_features: features,
        required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        default_queue: wgpu::QueueDescriptor::default(),
    };
    let (device, queue) = pollster::block_on(adapter.request_device(&desc))
        .map_err(|e| format!("GPU refused the 3D preview: {e}"))?;
    // A lost device or a validation slip must become an error message,
    // not the default handler's panic.
    device.on_uncaptured_error(Arc::new(move |e: wgpu::Error| {
        if let Ok(mut slot) = errors.lock() {
            *slot = Some(e.to_string());
        }
    }));
    Ok((device, queue))
}

fn globals(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("globals"),
        size: GLOBALS,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("globals"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("globals"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    (buffer, layout, bind)
}
