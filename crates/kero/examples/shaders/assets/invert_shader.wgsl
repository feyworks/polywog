// all params must be in @group(0), texture params look like this:
@group(0) @binding(0)
var perlin_texture: texture_2d<f32>;

// each param gets a new @binding, incrementing by one:
@group(0) @binding(1)
var perlin_sampler: sampler;

// uniform variables can be: f32, vec2f, vec3f, mat2f, mat3f, and mat4f
@group(0) @binding(2)
var<uniform> scroll: vec2f;

@vertex
fn vert_main(vert: Vertex) -> Fragment {
    return vert_default(vert);
}

@fragment
fn frag_main(frag: Fragment) -> @location(0) vec4f {
    // sample the main texture
    var pixel = textureSample(main_texture, main_sampler, frag.tex);

    // sampler the perlin noise and tweak it a bit
    var perlin = textureSample(perlin_texture, perlin_sampler, frag.tex + scroll).r;
    perlin = pow(1 - perlin, 2);

    // get the texture's inverted pixel color
    var invert = vec4(1 - pixel.rgb, pixel.a);

    // blend between regular/inverted by a factor determined by the perlin noise
    var output = pixel * (1 - perlin) + invert * perlin;

    // perform default frag shader on the result
    return apply_mode(output, frag.col, frag.mode);
}