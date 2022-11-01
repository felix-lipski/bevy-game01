pub mod graph;
use graph::{FlatPass3dNode, Flat3d};
use bevy::{
    utils::{ tracing::error, HashMap, HashSet, },
    pbr::{
        MeshPipeline, DrawMesh, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup, Material,
    },
    app::Plugin,
    asset::{load_internal_asset, Handle, HandleUntyped, AssetEvent, Assets},
    core_pipeline::core_3d::{
        Opaque3d, graph::node::MAIN_PASS, graph::input::VIEW_ENTITY, MainPass3dNode,
    },
    ecs::{prelude::*, reflect::ReflectComponent, entity::Entity, system::{lifetimeless::{Read, SQuery, SRes}, SystemParamItem}},
    reflect::{Reflect, TypeUuid, std_traits::ReflectDefault},
    render::{
        mesh::{Mesh, MeshVertexBufferLayout},
        render_asset::{RenderAssets, PrepareAssetLabel},
        render_phase::{
            AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline, EntityRenderCommand, RenderCommandResult, TrackedRenderPass,
            sort_phase_system
        },
        render_graph::{RenderGraph, SlotInfo, SlotType},
        render_resource::*,
        prelude::Image,
        view::{ExtractedView, Msaa, VisibleEntities},
        texture::FallbackImage,
        renderer::RenderDevice,
        RenderApp, RenderStage, Extract,
    },
};
use std::{hash::{Hash}, marker::PhantomData};

pub const WIREFRAME_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 192598014480025766);

#[derive(Debug)]
pub struct WireframePlugin<M: Material>(PhantomData<M>);

impl<M: Material> Default for WireframePlugin<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<M: Material> Plugin for WireframePlugin<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut bevy::app::App) {
        load_internal_asset!(
            app,
            WIREFRAME_SHADER_HANDLE,
            // "render/wireframe.wgsl",
            "render/merged.wgsl",
            Shader::from_wgsl
        );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<DrawFunctions<Flat3d>>()
                // .add_system_to_stage(RenderStage::Extract, extract_core_3d_camera_phases)
                // .add_system_to_stage(RenderStage::Prepare, prepare_core_3d_depth_textures)
                .add_system_to_stage(RenderStage::PhaseSort, sort_phase_system::<Flat3d>)
                .add_render_command::<Flat3d, DrawWireframes<M>>()
                .init_resource::<WireframePipeline<M>>()
                .init_resource::<ExtractedWireframes<M>>()
                .init_resource::<PreparedWireframes<M>>()
                .init_resource::<SpecializedMeshPipelines<WireframePipeline<M>>>()
                .add_system_to_stage(RenderStage::Extract, extract_wireframes::<M>)
                .add_system_to_stage(RenderStage::Prepare, prepare_materials::<M>.after(PrepareAssetLabel::PreAssetPrepare))
                .add_system_to_stage(RenderStage::Queue, queue_wireframes::<M>);

            let pass_node_3d = FlatPass3dNode::new(&mut render_app.world);
            let mut graph = render_app.world.resource_mut::<RenderGraph>();

            let mut draw_3d_graph = RenderGraph::default();
            draw_3d_graph.add_node(graph::FLAT_PASS, pass_node_3d);
            let input_node_id = draw_3d_graph.set_input(vec![SlotInfo::new(
                VIEW_ENTITY,
                SlotType::Entity,
            )]);
            draw_3d_graph
                .add_slot_edge(
                    input_node_id,
                    VIEW_ENTITY,
                    graph::FLAT_PASS,
                    MainPass3dNode::IN_VIEW,
                )
                .unwrap();
            graph.add_sub_graph(graph::NAME, draw_3d_graph);
            

            // let shadow_pass_node = ShadowPassNode::new(&mut render_app.world);
            // render_app.add_render_command::<Shadow, DrawShadowMesh>();
            // let mut graph = render_app.world.resource_mut::<RenderGraph>();
            // let draw_3d_graph = graph
            //     .get_sub_graph_mut(bevy_core_pipeline::core_3d::graph::NAME)
            //     .unwrap();
            // draw_3d_graph.add_node(draw_3d_graph::node::SHADOW_PASS, shadow_pass_node);
            // draw_3d_graph
            //     .add_node_edge(
            //         draw_3d_graph::node::SHADOW_PASS,
            //         bevy_core_pipeline::core_3d::graph::node::MAIN_PASS,
            //     )
            //     .unwrap();
            // draw_3d_graph
            //     .add_slot_edge(
            //         draw_3d_graph.input_node().unwrap().id,
            //         bevy_core_pipeline::core_3d::graph::input::VIEW_ENTITY,
            //         draw_3d_graph::node::SHADOW_PASS,
            //         ShadowPassNode::IN_VIEW,
            //     )
            //     .unwrap();
            // }
        }
    }
}

// fn extract_wireframes(mut commands: Commands, query: Extract<Query<Entity, With<Wireframe>>>) {
//     // for entity in query.iter() {
//     //     commands.get_or_spawn(entity).insert(Wireframe);
//     // }
// }

fn extract_wireframes<M: Material>(
    mut commands: Commands,
    mut events: Extract<EventReader<AssetEvent<M>>>,
    assets: Extract<Res<Assets<M>>>,
) {
    let mut changed_assets = HashSet::default();
    let mut removed = Vec::new();
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                changed_assets.insert(handle.clone_weak());
            }
            AssetEvent::Removed { handle } => {
                changed_assets.remove(handle);
                removed.push(handle.clone_weak());
            }
        }
    }

    let mut extracted_assets = Vec::new();
    for handle in changed_assets.drain() {
        if let Some(asset) = assets.get(&handle) {
            extracted_assets.push((handle, asset.clone()));
        }
    }

    commands.insert_resource(ExtractedWireframes {
        extracted: extracted_assets,
        // removed,
    });
}

/// Controls whether an entity should rendered in wireframe-mode if the [`WireframePlugin`] is enabled
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Wireframe;

pub struct WireframePipeline<M: Material> {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
    material_layout: BindGroupLayout,
    marker: PhantomData<M>,
}
impl<M: Material> FromWorld for WireframePipeline<M> {
    fn from_world(render_world: &mut World) -> Self {
        let render_device = render_world.resource::<RenderDevice>();
        WireframePipeline {
            mesh_pipeline: render_world.resource::<MeshPipeline>().clone(),
            shader: WIREFRAME_SHADER_HANDLE.typed(),
            material_layout: M::bind_group_layout(render_device),
            marker: PhantomData,
        }
    }
}

impl<M: Material> SpecializedMeshPipeline for WireframePipeline<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        // let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        // descriptor.vertex.shader = self.shader.clone_weak();
        // descriptor.fragment.as_mut().unwrap().shader = self.shader.clone_weak();
        // // descriptor.primitive.polygon_mode = PolygonMode::Line;
        // descriptor.primitive.polygon_mode = PolygonMode::Fill;
        // descriptor.depth_stencil.as_mut().unwrap().bias.slope_scale = 1.0;
        // Ok(descriptor)

        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        // descriptor.vertex.buffers.push(VertexBufferLayout {
        //     array_stride: std::mem::size_of::<InstanceData>() as u64,
        //     step_mode: VertexStepMode::Instance,
        //     attributes: vec![
        //         VertexAttribute {
        //             format: VertexFormat::Float32x4,
        //             offset: 0,
        //             shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
        //         },
        //         VertexAttribute {
        //             format: VertexFormat::Float32x4,
        //             offset: VertexFormat::Float32x4.size(),
        //             shader_location: 4,
        //         },
        //     ],
        // });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        // descriptor.layout = Some(vec![
        //     self.mesh_pipeline.view_layout.clone(),
        //     self.mesh_pipeline.mesh_layout.clone(),
        // ]);
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            // // self.mesh_pipeline.skinned_mesh_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);

        Ok(descriptor)
    }
}

#[allow(clippy::too_many_arguments)]
fn queue_wireframes<M: Material>(
    opaque_3d_draw_functions: Res<DrawFunctions<Flat3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    wireframe_pipeline: Res<WireframePipeline<M>>,
    prepared_wireframes: Res<PreparedWireframes<M>>,
    mut pipelines: ResMut<SpecializedMeshPipelines<WireframePipeline<M>>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    mut material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform)>,
    mut views: Query<(&ExtractedView, &VisibleEntities, &mut RenderPhase<Flat3d>)>,
) where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    let draw_custom = opaque_3d_draw_functions
        .read()
        .get_id::<DrawWireframes<M>>()
        .unwrap();
    for (view, visible_entities, mut opaque_phase) in &mut views {
        let rangefinder = view.rangefinder3d();

        let add_render_phase =
            |(entity, mesh_handle, mesh_uniform): (Entity, &Handle<Mesh>, &MeshUniform)| {
                if let Some(mesh) = render_meshes.get(mesh_handle) {
                    let key = MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                    let pipeline_id = pipelines.specialize(
                        &mut pipeline_cache,
                        &wireframe_pipeline,
                        key,
                        &mesh.layout,
                    );
                    let pipeline_id = match pipeline_id {
                        Ok(id) => id,
                        Err(err) => {
                            error!("{}", err);
                            return;
                        }
                    };
                    opaque_phase.add(Flat3d {
                        entity,
                        pipeline: pipeline_id,
                        draw_function: draw_custom,
                        distance: rangefinder.distance(&mesh_uniform.transform),
                    });
                }
            };

        visible_entities
            .entities
            .iter()
            .filter_map(|visible_entity| material_meshes.get(*visible_entity).ok())
            .for_each(add_render_phase);
    }
}

/// Sets the bind group for a given [`Material`] at the configured `I` index.
pub struct SetMaterialBindGroup2<M: Material, const I: usize>(PhantomData<M>);

impl<M: Material, const I: usize> EntityRenderCommand for SetMaterialBindGroup2<M, I> {
    type Param = (SRes<PreparedWireframes<M>>, SQuery<Read<Handle<M>>>);
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (materials, query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // for material_handle in query.iter() {
        //     println!("in SetMaterialBindGroup2, {}", material_handle.id == item);
        // };
        // let material_handle = query.get(item);
        match query.get(item) {
            Ok(material_handle) => {
                println!("OK");
                let material = materials.into_inner().get(material_handle).unwrap();
                pass.set_bind_group(I, &material.bind_group, &[]);
            }
            Err(_) => {
                println!("Err");
            }
        }
        // let material_handle = query.get(item).unwrap();
        // let material = materials.into_inner().get(material_handle).unwrap();
        // pass.set_bind_group(I, &material.bind_group, &[]);
        RenderCommandResult::Success
    }
}

struct ExtractedWireframes<M: Material> {
    extracted: Vec<(Handle<M>, M)>,
}

impl<M: Material> Default for ExtractedWireframes<M> {
    fn default() -> Self {
        Self {
            extracted: Default::default(),
        }
    }
}


/// Data prepared for a [`Material`] instance.
pub struct PreparedWireframe<T: Material> {
    pub bindings: Vec<OwnedBindingResource>,
    pub bind_group: BindGroup,
    pub key: T::Data,
}


/// Stores all prepared representations of [`Material`] assets for as long as they exist.
pub type PreparedWireframes<T> = HashMap<Handle<T>, PreparedWireframe<T>>;

/// All [`Material`] values of a given type that should be prepared next frame.
struct PrepareNextFrameMaterials<M: Material> {
    assets: Vec<(Handle<M>, M)>,
}

impl<M: Material> Default for PrepareNextFrameMaterials<M> {
    fn default() -> Self {
        Self {
            assets: Default::default(),
        }
    }
}

fn prepare_materials<M: Material>(
    mut prepare_next_frame: Local<PrepareNextFrameMaterials<M>>,
    mut extracted_assets: ResMut<ExtractedWireframes<M>>,
    mut render_materials: ResMut<PreparedWireframes<M>>,
    render_device: Res<RenderDevice>,
    images: Res<RenderAssets<Image>>,
    fallback_image: Res<FallbackImage>,
    pipeline: Res<WireframePipeline<M>>,
) {
    let mut queued_assets = std::mem::take(&mut prepare_next_frame.assets);
    for (handle, material) in queued_assets.drain(..) {
        match prepare_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }

    // for removed in std::mem::take(&mut extracted_assets.removed) {
    //     render_materials.remove(&removed);
    // }

    for (handle, material) in std::mem::take(&mut extracted_assets.extracted) {
        match prepare_material(
            &material,
            &render_device,
            &images,
            &fallback_image,
            &pipeline,
        ) {
            Ok(prepared_asset) => {
                println!("inserting");
                render_materials.insert(handle, prepared_asset);
            }
            Err(AsBindGroupError::RetryNextUpdate) => {
                prepare_next_frame.assets.push((handle, material));
            }
        }
    }
}

fn prepare_material<M: Material>(
    material: &M,
    render_device: &RenderDevice,
    images: &RenderAssets<Image>,
    fallback_image: &FallbackImage,
    pipeline: &WireframePipeline<M>,
) -> Result<PreparedWireframe<M>, AsBindGroupError> {
    let prepared = material.as_bind_group(
        &pipeline.material_layout,
        render_device,
        images,
        fallback_image,
    )?;
    Ok(PreparedWireframe {
        bindings: prepared.bindings,
        bind_group: prepared.bind_group,
        key: prepared.data,
    })
}

type DrawWireframes<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMaterialBindGroup2<M, 1>,
    SetMeshBindGroup<1>,
    DrawMesh,
);

