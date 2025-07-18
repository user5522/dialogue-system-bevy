mod dialogue;

use std::f32::consts::PI;

use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::{
    pbr::{CascadeShadowConfigBuilder, Material, MaterialPlugin},
    prelude::*,
};

use crate::dialogue::{DialoguePlugin, components::*};

pub const ORIGINAL_JOE_POSITION: Vec3 = Vec3::new(3.0, 0.0, 0.0);
const PLAYER_NAME: &str = "Joe Swanson";
const CHARACTER_MESH_RADIUS: f32 = 0.55;
const CHARACTER_MESH_HEIGHT: f32 = 2.0;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StripedMaterial {
    #[uniform(0)]
    top_color: LinearRgba,
    #[uniform(1)]
    middle_color: LinearRgba,
    #[uniform(2)]
    bottom_color: LinearRgba,
}

impl Material for StripedMaterial {
    fn fragment_shader() -> ShaderRef {
        "striped_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DialoguePlugin)
        .add_plugins(MaterialPlugin::<StripedMaterial>::default())
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StripedMaterial>>,
) {
    let capsule = meshes.add(Capsule3d::new(CHARACTER_MESH_RADIUS, CHARACTER_MESH_HEIGHT));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        DialogueCamera,
    ));

    commands.spawn((
        Mesh3d(capsule.clone()),
        MeshMaterial3d::<StripedMaterial>(materials.add(StripedMaterial {
            top_color: Color::srgb(0.9, 0.64, 0.61).into(),
            middle_color: Color::WHITE.into(),
            bottom_color: Color::srgb(0.2, 0.6, 0.2).into(),
        })),
        Transform::from_xyz(-3.0, 0.0, 0.0),
        DialogueTarget,
        Speaker {
            name: "Peter Griffin".to_string(),
        },
    ));

    commands.spawn((
        Mesh3d(capsule.clone()),
        MeshMaterial3d::<StripedMaterial>(materials.add(StripedMaterial {
            top_color: Color::srgb(0.95, 0.78, 0.59).into(),
            middle_color: Color::srgb(0.89, 0.16, 0.11).into(),
            bottom_color: Color::srgb(0., 0.24, 0.33).into(),
        })),
        Transform::from_xyz(-1.0, 0.0, 0.0),
        DialogueTarget,
        Speaker {
            name: "Glenn Quagmire".to_string(),
        },
    ));

    commands.spawn((
        Mesh3d(capsule.clone()),
        MeshMaterial3d::<StripedMaterial>(materials.add(StripedMaterial {
            top_color: Color::srgb(0.63, 0.34, 0.2).into(),
            middle_color: Color::srgb(0.96, 0.86, 0.4).into(),
            bottom_color: Color::srgb(0.15, 0.18, 0.41).into(),
        })),
        Transform::from_xyz(1.0, 0.0, 0.0),
        DialogueTarget,
        Speaker {
            name: "Cleveland Brown".to_string(),
        },
    ));

    commands.spawn((
        Mesh3d(capsule.clone()),
        MeshMaterial3d::<StripedMaterial>(materials.add(StripedMaterial {
            top_color: Color::srgb(0.96, 0.64, 0.51).into(),
            middle_color: Color::srgb(0.72, 0.72, 0.75).into(),
            bottom_color: Color::srgb(0.14, 0.25, 0.38).into(),
        })),
        Transform::from_translation(ORIGINAL_JOE_POSITION),
        DialogueTarget,
        Speaker {
            name: PLAYER_NAME.to_string(),
        },
        Actor {
            name: PLAYER_NAME.to_string(),
        },
    ));
}
