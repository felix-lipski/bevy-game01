use std::cmp::Reverse;
use bevy::{
    ecs::prelude::*,
    render::{
        camera::{Camera, ExtractedCamera},
        render_graph::{Node, NodeRunError, RenderGraphContext, SlotInfo, SlotType},
        render_phase::{ DrawFunctions, TrackedRenderPass, CachedRenderPipelinePhaseItem, EntityPhaseItem, DrawFunctionId, PhaseItem, RenderPhase, },
        render_resource::{ LoadOp, Operations, RenderPassDepthStencilAttachment, RenderPassDescriptor, CachedRenderPipelineId, },
        renderer::RenderContext,
        view::{ViewDepthTexture, ViewTarget, ExtractedView},
    },
    utils::{ FloatOrd, },
    core_pipeline::{
        clear_color::{ClearColor, ClearColorConfig},
        core_3d::{
            Camera3d,
            // Opaque3d,
        },
    },
};
// #[cfg(feature = "trace")]

pub struct FlatPass3dNode {
    query: QueryState<
        (
            &'static ExtractedCamera,
            // &'static RenderPhase<Opaque3d>,
            // &'static RenderPhase<AlphaMask3d>,
            // &'static RenderPhase<Transparent3d>,
            &'static RenderPhase<Flat3d>,
            &'static Camera3d,
            &'static ViewTarget,
            &'static ViewDepthTexture,
        ),
        With<ExtractedView>,
    >,
}

impl FlatPass3dNode {
    pub const IN_VIEW: &'static str = "view";

    pub fn new(world: &mut World) -> Self {
        Self {
            query: world.query_filtered(),
        }
    }
}

impl Node for FlatPass3dNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![SlotInfo::new(FlatPass3dNode::IN_VIEW, SlotType::Entity)]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let view_entity = graph.get_input_entity(Self::IN_VIEW)?;
        // let (camera, opaque_phase, alpha_mask_phase, transparent_phase, camera_3d, target, depth) =
        let (camera, flat_phase, camera_3d, target, depth) =
            match self.query.get_manual(world, view_entity) {
                Ok(query) => query,
                Err(_) => {
                    return Ok(());
                } // No window
            };

        // Always run opaque pass to ensure screen is cleared
        {
            // Run the opaque pass, sorted front-to-back
            // NOTE: Scoped to drop the mutable borrow of render_context
            #[cfg(feature = "trace")]
            let _main_opaque_pass_3d_span = info_span!("main_opaque_pass_3d").entered();
            let pass_descriptor = RenderPassDescriptor {
                label: Some("main_opaque_pass_3d"),
                // NOTE: The opaque pass loads the color
                // buffer as well as writing to it.
                color_attachments: &[Some(target.get_color_attachment(Operations {
                    load: match camera_3d.clear_color {
                        ClearColorConfig::Default => {
                            LoadOp::Clear(world.resource::<ClearColor>().0.into())
                        }
                        ClearColorConfig::Custom(color) => LoadOp::Clear(color.into()),
                        ClearColorConfig::None => LoadOp::Load,
                    },
                    store: true,
                }))],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth.view,
                    // NOTE: The opaque main pass loads the depth buffer and possibly overwrites it
                    depth_ops: Some(Operations {
                        // NOTE: 0.0 is the far plane due to bevy's use of reverse-z projections.
                        load: camera_3d.depth_load_op.clone().into(),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            };

            // let draw_functions = world.resource::<DrawFunctions<Opaque3d>>();
            let draw_functions = world.resource::<DrawFunctions<Flat3d>>();

            let render_pass = render_context
                .command_encoder
                .begin_render_pass(&pass_descriptor);
            let mut draw_functions = draw_functions.write();
            let mut tracked_pass = TrackedRenderPass::new(render_pass);
            if let Some(viewport) = camera.viewport.as_ref() {
                tracked_pass.set_camera_viewport(viewport);
            }
            // for item in &opaque_phase.items {
            for item in &flat_phase.items {
                let draw_function = draw_functions.get_mut(item.draw_function).unwrap();
                draw_function.draw(world, &mut tracked_pass, view_entity, item);
            }
        }
        Ok(())
    }
}


pub struct Flat3d {
    pub distance: f32,
    pub pipeline: CachedRenderPipelineId,
    pub entity: Entity,
    pub draw_function: DrawFunctionId,
}

impl PhaseItem for Flat3d {
    // NOTE: Values increase towards the camera. Front-to-back ordering for opaque means we need a descending sort.
    type SortKey = Reverse<FloatOrd>;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        Reverse(FloatOrd(self.distance))
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        // Key negated to match reversed SortKey ordering
        radsort::sort_by_key(items, |item| -item.distance);
    }
}

impl EntityPhaseItem for Flat3d {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl CachedRenderPipelinePhaseItem for Flat3d {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}
