@vertex
fn vert_main(vert: Vertex) -> Fragment {
    return vert_default(vert);
}

@fragment
fn frag_main(frag: Fragment) -> @location(0) vec4f {
    return frag_default(frag);
}