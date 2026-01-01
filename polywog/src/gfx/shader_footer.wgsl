struct Vertex {
    @location(0) pos: vec2f,
    @location(1) tex: vec2f,
    @location(2) col: vec4f,
    @location(3) mode: vec4f,
};

struct Fragment {
    @builtin(position) pos: vec4f,
    @location(0) tex: vec2f,
    @location(1) col: vec4f,
    @location(2) mode: vec4f,
};

@group(0) @binding($0)
var<uniform> view_matrix: mat4x4f;

@group(0) @binding($1)
var main_texture: texture_2d<f32>;

@group(0) @binding($2)
var main_sampler: sampler;

fn vert_default(vert: Vertex) -> Fragment {
    var frag: Fragment;
    frag.pos = view_matrix * vec4f(vert.pos, 0.0, 1.0);
    frag.tex = vert.tex;
    frag.col = vert.col;
    frag.mode = vert.mode;
    return frag;
}

fn frag_default(frag: Fragment) -> vec4f {
    var pixel = textureSample(main_texture, main_sampler, frag.tex);
    return apply_mode(pixel, frag.col, frag.mode);
}

fn apply_mode(pixel: vec4f, color: vec4f, mode: vec4f) -> vec4f {
    return
        (mode.x * pixel * color) +   // mult
        (mode.y * pixel.a * color) + // wash
        (mode.z * color);            // veto
}