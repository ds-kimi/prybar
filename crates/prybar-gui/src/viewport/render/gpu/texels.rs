//! Texture upload and the small built-in images.

use prybar_vtf::Image;

use super::Gpu;

/// Layout slot: the texture (`binding` 0) or its sampler.
pub fn entry(binding: u32, texture: bool) -> wgpu::BindGroupLayoutEntry {
    let ty = if texture {
        wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    } else {
        wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
    };
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty,
        count: None,
    }
}

/// `levels` halve in size each step, as VTF mips do.
pub fn upload(gpu: &Gpu, levels: &[Image], srgb: bool) -> wgpu::TextureView {
    let size = |i: &Image| wgpu::Extent3d {
        width: i.width,
        height: i.height,
        depth_or_array_layers: 1,
    };
    let format = if srgb {
        wgpu::TextureFormat::Rgba8UnormSrgb
    } else {
        wgpu::TextureFormat::Rgba8Unorm
    };
    let tex = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("material"),
        size: size(&levels[0]),
        mip_level_count: levels.len() as u32,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    write(gpu, &tex, levels);
    tex.create_view(&Default::default())
}

fn write(gpu: &Gpu, tex: &wgpu::Texture, levels: &[Image]) {
    for (mip, img) in levels.iter().enumerate() {
        let at = wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: mip as u32,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };
        let layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(img.width * 4),
            rows_per_image: None,
        };
        let size = wgpu::Extent3d {
            width: img.width,
            height: img.height,
            depth_or_array_layers: 1,
        };
        gpu.queue.write_texture(at, &img.rgba, layout, size);
    }
}

/// 8x8 two-grey checker: reads as "missing" without shouting.
pub fn checker() -> Vec<u8> {
    (0..64)
        .flat_map(|i| {
            let v = if (i % 8 < 4) == (i / 8 < 4) { 70 } else { 110 };
            [v, v, v, 255]
        })
        .collect()
}
