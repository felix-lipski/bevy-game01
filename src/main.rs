use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(160.0, 0.0, 0.07)))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(lock_pointer)
        .add_system(player_movement)
        .add_system(player_head_rotate)
        .add_system(player_body_rotate)
        .run();
}

#[derive(Component)]
struct PlayerBody;

#[derive(Component)]
struct PlayerHead;

#[derive(Component)]
struct Position(Vec3);

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerBody, &mut Transform)>,
) {
    let velo_f = 2.0;
    for (_, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            let frw = transform.forward().reject_from(Vec3::Y).normalize() * time.delta_seconds() * velo_f;
            transform.translation += frw;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            let frw = transform.back().reject_from(Vec3::Y).normalize() * time.delta_seconds() * velo_f;
            transform.translation += frw;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            let frw = transform.left() * time.delta_seconds() * velo_f;
            transform.translation += frw;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            let frw = transform.right() * time.delta_seconds() * velo_f;
            transform.translation += frw;
        }
        if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::LShift) {
            transform.translation.y += time.delta_seconds() * velo_f;
        }
        if keyboard_input.pressed(KeyCode::LControl) || keyboard_input.pressed(KeyCode::LControl) {
            transform.translation.y -= time.delta_seconds() * velo_f;
        }
        if keyboard_input.pressed(KeyCode::E) || keyboard_input.pressed(KeyCode::E) {
            transform.rotate_y(-time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::Q) || keyboard_input.pressed(KeyCode::Q) {
            transform.rotate_y(time.delta_seconds());
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
        let delta_x = mouse_motion_event.delta.x;
        rotation_x += delta_x;
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
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..default()
        })
        .insert(PlayerBody)
        .with_children(|parent| {
            parent.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            })
            .insert(PlayerHead);

        });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::hsl(77.0, 1.0, 0.66).into()),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::hsl(216.0, 1.0, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
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
