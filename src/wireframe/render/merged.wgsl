#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

//@group(1) @binding(1)
//var base_color_texture: texture_2d<f32>;
//@group(1) @binding(2)
//var base_color_sampler: sampler;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
//    @location(1) normal: vec3<f32>,
//#ifdef VERTEX_UVS
//    @location(2) uv: vec2<f32>,
//#endif
//#ifdef VERTEX_TANGENTS
//    @location(3) tangent: vec4<f32>,
//#endif
//#ifdef VERTEX_COLORS
//    @location(4) color: vec4<f32>,
//#endif
//#ifdef SKINNED
//    @location(5) joint_indices: vec4<u32>,
//    @location(6) joint_weights: vec4<f32>,
//#endif
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    //@location(0) normal: vec3<f32>,
    //@location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let model = mesh.model;
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    //out.uv = vertex.uv;
    //out.normal = vertex.normal;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    //let u1 = base_color_texture;
    //let u2 = base_color_sampler;
    //let u3 = in.uv;
    //let out = textureSample(base_color_texture, base_color_sampler, in.uv);

    return vec4<f32>(0.0, 0.3, 1.0, 1.0);
    //return vec4<f32>(in.normal, 1.0);
    //out.x = 1.0;
    //return out;
}

//fn fragment0(
//    #import bevy_pbr::mesh_vertex_output
//) -> @location(0) vec4<f32> {
//    return textureSample(base_color_texture, base_color_sampler, uv);
//}
