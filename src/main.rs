use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfig, prelude::*, window::CursorGrabMode};
use bevy_aabb_instancing::VertexPullingRenderPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use chunk::Chunk;

use crate::{block::*, chunk::initialize_example_chunk};

mod block;
mod camera;
mod chunk;
mod event;
mod ui;
mod util;

fn main() {
    println!("Application initializing.");

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(ui::UserInterfacePlugin)
        .add_plugin(event::EventSystemPlugin)
        .add_plugin(BlockPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin {
        //     always_on_top: true,
        //     enabled: true,
        //     mode: DebugRenderMode::COLLIDER_SHAPES,
        //     ..default()
        // })
        .add_plugin(VertexPullingRenderPlugin { outlines: true })
        .add_startup_system(setup)
        .add_startup_system(print_resources)
        .add_startup_system(initialize_example_chunk)
        .add_system(cursor_grab_system)
        .add_system(fixed.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(5.0))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.0, 50.0));
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfig { ..default() },
        ..default()
    });

    Chunk::get_some_noise();
}

fn fixed() {
    println!("running fixed system")
}

fn print_resources(world: &World) {
    let components = world.components();

    let mut r: Vec<_> = world
        .storages()
        .resources
        .iter()
        .map(|(id, _)| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}

// TODO capture cursor in screen and dont let it get out
fn cursor_grab_system(mut windows: Query<&mut Window>, key: Res<Input<KeyCode>>) {
    let mut window = windows.get_single_mut().unwrap();

    if key.just_pressed(KeyCode::LControl) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;

        println!("locking cursor");
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
