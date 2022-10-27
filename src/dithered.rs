use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError
        },
    },
};

#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct DitheredMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}
impl Material for DitheredMaterial {
    fn vertex_shader() -> ShaderRef { "shaders/mask.vert".into() }
    fn fragment_shader() -> ShaderRef { "shaders/mask.frag".into() }
    fn specialize( _pipeline: &MaterialPipeline<Self>, descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout, _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
