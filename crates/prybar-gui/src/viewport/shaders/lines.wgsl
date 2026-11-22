// Ground grid and bone overlay: colored lines, alpha blended.

struct Globals {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    mode: vec4<f32>,
    wire: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) color: vec4<f32>) -> VOut {
    var out: VOut;
    out.pos = g.view_proj * vec4<f32>(p, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs(in: VOut) -> @location(0) vec4<f32> {
    return in.color;
}
