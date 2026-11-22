//! One texture bind group per material slot, plus the two stand-ins:
//! clay while textures load or when they are off, a checker when missing.

use prybar_vtf::Image;

use super::texels::{checker, entry};
use super::Gpu;

pub struct Materials {
    pub layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    clay: wgpu::BindGroup,
    missing: wgpu::BindGroup,
    /// `None` until a set arrives; inside, `None` marks a missing texture.
    slots: Option<Vec<Option<wgpu::BindGroup>>>,
}

/// Linear clay from before textures, kept as the untextured look.
const CLAY: [u8; 4] = [158, 153, 145, 255];

impl Materials {
    pub fn new(gpu: &Gpu) -> Self {
        let d = &gpu.device;
        let layout = d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("material"),
            entries: &[entry(0, true), entry(1, false)],
        });
        let sampler = d.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            anisotropy_clamp: 4,
            ..Default::default()
        });
        let flat = |w, rgba: Vec<u8>| Image {
            width: w,
            height: w,
            rgba,
        };
        let clay = group(gpu, &layout, &sampler, &[flat(1, CLAY.to_vec())], false);
        let missing = group(gpu, &layout, &sampler, &[flat(8, checker())], false);
        Self {
            layout,
            sampler,
            clay,
            missing,
            slots: None,
        }
    }

    /// `None` levels draw the checker; an empty list goes back to clay.
    pub fn set(&mut self, gpu: &Gpu, slots: &[Vec<Image>]) {
        self.slots = (!slots.is_empty()).then(|| {
            slots
                .iter()
                .map(|l| (!l.is_empty()).then(|| group(gpu, &self.layout, &self.sampler, l, true)))
                .collect()
        });
    }

    pub fn get(&self, slot: usize) -> &wgpu::BindGroup {
        match &self.slots {
            None => &self.clay,
            Some(s) => s
                .get(slot)
                .and_then(Option::as_ref)
                .unwrap_or(&self.missing),
        }
    }
}

/// Uploads every level; textures are sRGB, the stand-ins are already linear.
fn group(
    gpu: &Gpu,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    levels: &[Image],
    srgb: bool,
) -> wgpu::BindGroup {
    let view = super::texels::upload(gpu, levels, srgb);
    gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("material"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    })
}
