// Key, fill and rim lights over the base texture. Untextured parts get a
// flat clay texture, which shows shape better than a guessed color.

struct Globals {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    // x: 1 shows normals as colors.
    mode: vec4<f32>,
    wire: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(1) @binding(0) var base_tex: texture_2d<f32>;
@group(1) @binding(1) var base_smp: sampler;

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) n: vec3<f32>, @location(2) uv: vec2<f32>) -> VOut {
    var out: VOut;
    out.pos = g.view_proj * vec4<f32>(p, 1.0);
    out.normal = n;
    out.world = p;
    out.uv = uv;
    return out;
}

@fragment
fn fs(in: VOut) -> @location(0) vec4<f32> {
    // Sampled first: derivatives must come from uniform control flow.
    let base = textureSample(base_tex, base_smp, in.uv);
    // Opaque materials arrive with alpha 255, so only cutouts drop here.
    if (base.a < 0.5) {
        discard;
    }
    let face = normalize(cross(dpdx(in.world), dpdy(in.world)));
    var n = in.normal;
    // Broken files ship zero normals; the face normal still reads well.
    if (dot(n, n) < 1e-6) {
        n = face;
    }
    n = normalize(n);
    let v = normalize(g.eye.xyz - in.world);
    // Source models are single sided; light the back faces as seen.
    if (dot(n, v) < 0.0) {
        n = -n;
    }
    if (g.mode.x > 0.5) {
        return vec4<f32>(n * 0.5 + 0.5, 1.0);
    }
    let key = max(dot(n, normalize(vec3<f32>(0.5, 0.35, 0.8))), 0.0);
    let fill = max(dot(n, normalize(vec3<f32>(-0.6, -0.5, 0.25))), 0.0);
    let rim = pow(1.0 - max(dot(n, v), 0.0), 3.0);
    let light = 0.16 + key * 0.78 + fill * 0.22 + rim * 0.3;
    return vec4<f32>(base.rgb * light, 1.0);
}

// Wire overlay: one flat color over the shaded pass.
@fragment
fn fs_wire(in: VOut) -> @location(0) vec4<f32> {
    return g.wire;
}
