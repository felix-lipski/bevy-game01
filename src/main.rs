mod movement;
use movement::{MovementPlugin, PlayerHead, PlayerBody};
mod dithered;
use dithered::{DitheredMaterial, mod_scene};
use bevy::{
    prelude::*,
    render::{
        render_resource::{ SamplerDescriptor, FilterMode, },
        texture::ImageSettings,
    },
};
use bevy_rapier3d::prelude::*;
use bevy::log::LogPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::hsl(160.0, 0.0, 0.07)));
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ImageSettings { default_sampler: SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()}
        });
    app.add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>());
    app.add_plugin(MaterialPlugin::<DitheredMaterial>::default());
    app.add_plugin(MovementPlugin);
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_system(mod_scene);
    app.add_startup_system(setup);
    app.run();

    let dot = bevy_mod_debugdump::get_render_graph(&mut app);
    std::fs::write("render-graph.dot", dot).expect("Failed to write render-graph.dot");
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
