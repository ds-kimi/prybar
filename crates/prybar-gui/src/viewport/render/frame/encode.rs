use super::color::linear;
use super::upload::SceneBuffers;
use crate::viewport::render::gpu::material::Materials;
use crate::viewport::render::gpu::pipelines::Pipes;
use crate::viewport::render::gpu::targets::Targets;
use crate::viewport::render::Request;

/// One MSAA pass resolved into the copyable texture.
pub fn pass(
    enc: &mut wgpu::CommandEncoder,
    t: &Targets,
    p: &Pipes,
    b: &SceneBuffers,
    req: &Request,
    (globals, materials): (&wgpu::BindGroup, &Materials),
) {
    let mut pass = begin(enc, t, req.clear);
    pass.set_bind_group(0, globals, &[]);
    // Every pipeline shares the layout, so group 1 must be set before
    // the grid too, even though the line shader never reads it.
    pass.set_bind_group(1, materials.get(usize::MAX), &[]);
    draw(&mut pass, p, b, req, materials);
}

fn begin<'a>(
    enc: &'a mut wgpu::CommandEncoder,
    t: &Targets,
    clear: [f32; 4],
) -> wgpu::RenderPass<'a> {
    let [r, g, b, _] = linear(clear).map(f64::from);
    let color = wgpu::Color { r, g, b, a: 1.0 };
    enc.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("viewport"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &t.msaa,
            resolve_target: Some(&t.resolve_view),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Discard,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &t.depth,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Discard,
            }),
            stencil_ops: None,
        }),
        ..Default::default()
    })
}

fn draw(pass: &mut wgpu::RenderPass, p: &Pipes, b: &SceneBuffers, req: &Request, m: &Materials) {
    pass.set_pipeline(&p.grid);
    pass.set_vertex_buffer(0, b.grid.slice(..));
    pass.draw(0..b.grid_points.len() as u32, 0..1);
    pass.set_index_buffer(b.index.slice(..), wgpu::IndexFormat::Uint32);
    pass.set_vertex_buffer(0, b.vertex.slice(..));
    pass.set_pipeline(&p.model);
    for part in &b.parts {
        pass.set_bind_group(1, m.get(part.material), &[]);
        pass.draw_indexed(part.first..part.first + part.count, 0, 0..1);
    }
    if let (true, Some(wire)) = (req.flags.wire, &p.wire) {
        pass.set_pipeline(wire);
        pass.draw_indexed(0..b.indices, 0, 0..1);
    }
    if req.flags.bones && b.bone_vertices > 0 {
        pass.set_pipeline(&p.bones);
        pass.set_vertex_buffer(0, b.bones.slice(..));
        pass.draw(0..b.bone_vertices, 0..1);
    }
}
