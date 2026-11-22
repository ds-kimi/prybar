use super::states::{self, Kind};
use super::targets::{COLOR, SAMPLES};
use super::Gpu;
use crate::viewport::shaders;

pub struct Pipes {
    pub model: wgpu::RenderPipeline,
    pub wire: Option<wgpu::RenderPipeline>,
    /// Grid: depth tested so the model hides it.
    pub grid: wgpu::RenderPipeline,
    /// Bones: drawn on top, the point of the overlay is seeing inside.
    pub bones: wgpu::RenderPipeline,
}

const MODEL_ATTRS: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];
const LINE_ATTRS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

const TARGET: wgpu::ColorTargetState = wgpu::ColorTargetState {
    format: COLOR,
    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
    write_mask: wgpu::ColorWrites::ALL,
};

pub fn build(gpu: &Gpu, material: &wgpu::BindGroupLayout) -> Pipes {
    let (layout, m, l) = setup(gpu, material);
    let tri = wgpu::PrimitiveTopology::TriangleList;
    let line = wgpu::PrimitiveTopology::LineList;
    let kind = |module, attrs, topology, polygon, depth| Kind {
        layout: &layout,
        module,
        attrs,
        topology,
        polygon,
        depth,
        fs: if polygon == wgpu::PolygonMode::Line {
            "fs_wire"
        } else {
            "fs"
        },
    };
    let fill = wgpu::PolygonMode::Fill;
    let wire = wgpu::PolygonMode::Line;
    Pipes {
        model: pipeline(gpu, &kind(&m, &MODEL_ATTRS, tri, fill, true)),
        wire: (gpu.wire).then(|| pipeline(gpu, &kind(&m, &MODEL_ATTRS, tri, wire, true))),
        grid: pipeline(gpu, &kind(&l, &LINE_ATTRS, line, fill, true)),
        bones: pipeline(gpu, &kind(&l, &LINE_ATTRS, line, fill, false)),
    }
}

/// The shared layout (globals, then the material) and both shader modules.
fn setup(
    gpu: &Gpu,
    material: &wgpu::BindGroupLayout,
) -> (wgpu::PipelineLayout, wgpu::ShaderModule, wgpu::ShaderModule) {
    let module = |name, src: &str| {
        gpu.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(name),
                source: wgpu::ShaderSource::Wgsl(src.into()),
            })
    };
    let layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&gpu.layout), Some(material)],
            immediate_size: 0,
        });
    (
        layout,
        module("model", shaders::MODEL),
        module("lines", shaders::LINES),
    )
}

fn pipeline(gpu: &Gpu, k: &Kind) -> wgpu::RenderPipeline {
    let buffers = states::buffers(k);
    gpu.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(k.layout),
            vertex: wgpu::VertexState {
                module: k.module,
                entry_point: Some("vs"),
                buffers: &buffers,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: k.module,
                entry_point: Some(k.fs),
                targets: &[Some(TARGET)],
                compilation_options: Default::default(),
            }),
            primitive: states::primitive(k),
            depth_stencil: Some(states::depth(k)),
            multisample: wgpu::MultisampleState {
                count: SAMPLES,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
}
