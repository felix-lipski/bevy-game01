use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, SamplerDescriptor, FilterMode
        },
        texture::ImageSettings,
        render_graph::RenderGraph
    },
};
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;
use bevy::log::LogPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::hsl(160.0, 0.0, 0.07)));
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ImageSettings { default_sampler: SamplerDescriptor {
            label: Some("Present Sampler"),
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()}
        });
    // app.add_plugins(DefaultPlugins);
    app.add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>());
    app.add_plugin(MaterialPlugin::<DitheredMaterial>::default());
    app.add_startup_system(setup);
    app.add_startup_system(lock_pointer);
    app.add_system(player_movement);
    app.add_system(player_jump);
    app.add_system(player_head_rotate);
    app.add_system(player_body_rotate);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    // app.add_plugin(RapierDebugRenderPlugin::default())
    app.run();

    let dot = bevy_mod_debugdump::get_render_graph(&mut app);
    std::fs::write("render-graph.dot", dot).expect("Failed to write render-graph.dot");;
}

#[derive(Component)]
struct PlayerBody;

#[derive(Component)]
struct PlayerHead;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerBody, &Transform, &mut Velocity)>,
) {
    let velo_f = 8.0;
    for (_, transform, mut vel) in query.iter_mut() {
        let mut linear_vel = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            linear_vel += transform.forward();
        }
        if keyboard_input.pressed(KeyCode::S) {
            linear_vel += transform.back();
        }
        if keyboard_input.pressed(KeyCode::A) {
            linear_vel += transform.left();
        }
        if keyboard_input.pressed(KeyCode::D) {
            linear_vel += transform.right();
        }
        linear_vel = linear_vel.normalize_or_zero() * velo_f;
        vel.linvel.x = linear_vel.x;
        vel.linvel.z = linear_vel.z;
    }
}

fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &PlayerBody, &Transform, &mut Velocity)>,
) {
    for (ent, _, transform, mut vel) in query.iter_mut() {
        if let Some((_, _)) = rapier_context.cast_ray(
            transform.translation,
            transform.down(),
            2.0,
            true,
            QueryFilter::new().exclude_collider(ent)
        ) {
            if keyboard_input.pressed(KeyCode::Space) {
                vel.linvel.y = 3.0;
            }
        }
    }
}

fn lock_pointer(mut windows: ResMut<Windows>) {
    debug!("Locking cursor");
    let window = windows.get_primary_mut().expect(
        "Expected to find window while locking pointer for FirstPersonControlPlugin. None found!",
    );
    window.set_cursor_position(Vec2::new(window.width() / 2f32, window.height() / 2f32));
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);
}

fn player_body_rotate(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&PlayerBody, &mut Transform)>,
) {
    let mut rotation_x = 0f32;
    for mouse_motion_event in mouse_motion_events.iter() {
        rotation_x += mouse_motion_event.delta.x;
    }
    for (_, mut transform) in query.iter_mut() {
        transform.rotate_y(rotation_x * -0.005);
    }
}

fn player_head_rotate(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&PlayerHead, &mut Transform)>,
) {
    let mut rotation_y = 0f32;
    for mouse_motion_event in mouse_motion_events.iter() {
        let delta_y = mouse_motion_event.delta.y;
        rotation_y += delta_y;
    }
    for (_, mut transform) in query.iter_mut() {
        let left = transform.left();
        transform.rotate_axis(left, rotation_y * 0.005);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<DitheredMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 1.0, 3.0),
            ..default()
        })
        .insert(PlayerBody)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(0.7, 0.25))
        .with_children(|parent| {
            parent
                .spawn_bundle(Camera3dBundle {
                    // transform: Transform::from_xyz(0.0, 3.0, 4.0),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                })
                .insert(PlayerHead);
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
            material: materials.add(Color::hsl(77.0, 0.5, 0.66).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(20.0, 0.01, 20.0));

    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: custom_materials.add(DitheredMaterial {
                color_texture: Some(asset_server.load("textures/regions.png")),
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 1.0, sectors: 32, stacks: 16 })),
            material: custom_materials.add(DitheredMaterial {
                color_texture: Some(asset_server.load("textures/regions.png")),
            }),
            transform: Transform::from_xyz(3.0, 1.5, 1.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0));
    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: custom_materials.add(DitheredMaterial {
                color_texture: Some(asset_server.load("textures/regions.png")),
            }),
            transform: Transform::from_xyz(0.7, 2.5, 0.6),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/skyscraper.glb#Scene0"),
        transform: Transform::from_xyz(-10.0, 0.0, -10.0),
        ..Default::default()
    });
}


#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct DitheredMaterial {
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
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

