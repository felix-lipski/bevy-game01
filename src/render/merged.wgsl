#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let model = mesh.model;
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    return out;
}

@fragment
fn fragment(
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.3, 1.0, 1.0);
    // return vec4<f32>(uv, 1.0);
}

fn fragment0(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    return textureSample(base_color_texture, base_color_sampler, uv);
}
