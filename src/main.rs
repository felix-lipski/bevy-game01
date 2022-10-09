use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(160.0, 0.0, 0.07)))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(lock_pointer)
        .add_system(player_movement)
        .add_system(player_jump)
        .add_system(player_head_rotate)
        .add_system(player_body_rotate)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
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
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::hsl(77.0, 1.0, 0.66).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 0.01, 5.0));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::hsl(77.0, 1.0, 0.5).into()),
            transform: Transform::from_xyz(-2.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));


    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::hsl(216.0, 1.0, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::hsl(216.0, 1.0, 0.5).into()),
            transform: Transform::from_xyz(0.7, 2.5, 0.6),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
