use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{
            MeshVertexBufferLayout, VertexAttributeValues,
        },
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError
        },
    },
};

#[derive(Component)]
pub struct Inserted;

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

pub fn mod_scene(
    mut commands: Commands,
    spheres: Query<
        (Entity, &Handle<Mesh>, &Name, &Handle<StandardMaterial>),
        Without<Inserted>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<DitheredMaterial>>,
    standard_materials: Res<Assets<StandardMaterial>>,
) {
    for sphere in spheres.iter() {
        let mesh = meshes.get_mut(sphere.1).unwrap();
        if let Some(VertexAttributeValues::Float32x3(
            positions,
        )) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[r, g, b]| {
                    [
                        (1. - *r) / 2.,
                        (1. - *g) / 2.,
                        (1. - *b) / 2.,
                        1.,
                    ]
                })
                .collect();
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            );
        }

        let custom_material =
            custom_materials.add(DitheredMaterial {
                color_texture: standard_materials
                    .get(sphere.3)
                    .and_then(|x| x.base_color_texture.clone()),
            });
        commands
            .entity(sphere.0)
            .remove::<Handle<StandardMaterial>>();
        commands.entity(sphere.0).insert(custom_material);
    }
}
