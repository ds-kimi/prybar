use super::targets::DEPTH;

/// How one pipeline differs from the others.
pub struct Kind<'a> {
    pub layout: &'a wgpu::PipelineLayout,
    pub module: &'a wgpu::ShaderModule,
    pub attrs: &'a [wgpu::VertexAttribute],
    pub topology: wgpu::PrimitiveTopology,
    pub polygon: wgpu::PolygonMode,
    pub depth: bool,
    pub fs: &'static str,
}

/// One interleaved vertex buffer, stride from the attributes.
pub fn buffers<'a>(k: &Kind<'a>) -> [Option<wgpu::VertexBufferLayout<'a>>; 1] {
    [Some(wgpu::VertexBufferLayout {
        array_stride: k.attrs.iter().map(|a| a.format.size()).sum(),
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: k.attrs,
    })]
}

pub fn primitive(k: &Kind) -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: k.topology,
        polygon_mode: k.polygon,
        ..Default::default()
    }
}

/// Wire lines sit on the surface they outline; a small bias keeps them
/// from flickering against the filled pass.
pub fn depth(k: &Kind) -> wgpu::DepthStencilState {
    let wire = k.polygon == wgpu::PolygonMode::Line;
    let bias = wgpu::DepthBiasState {
        constant: -2,
        slope_scale: -1.0,
        clamp: 0.0,
    };
    wgpu::DepthStencilState {
        format: DEPTH,
        depth_write_enabled: Some(k.depth && !wire),
        depth_compare: Some(if k.depth {
            wgpu::CompareFunction::LessEqual
        } else {
            wgpu::CompareFunction::Always
        }),
        stencil: wgpu::StencilState::default(),
        bias: if wire { bias } else { Default::default() },
    }
}
