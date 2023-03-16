use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseButtonInput,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
    winit::{WinitPlugin, WinitSettings, WinitWindows},
};
use smooth_bevy_cameras::LookTransform;

mod camera;
mod ui;

fn main() {
    println!("Application initializing.");

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(ui::UserInterfacePlugin)
        .add_startup_system(setup)
        .add_startup_system(print_resources)
        // .add_system(fps_update_system)
        .add_system(mouse_button_events)
        .add_system(cursor_grab_system)
        .add_system(fixed.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(5.0))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Bundle)]
struct Block {
    data: PbrBundle,
}

fn fixed() {
    println!("running fixed system")
}

fn mouse_button_events(
    windows: Query<&mut Window>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: Local<f32>,
    camera_query: Query<&LookTransform>,
    objects: Query<&Handle<Mesh>>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left {
                    println!("Click.");

                    let window = windows.get_single().unwrap();
                    let cursor = window.cursor_position().unwrap();

                    let target = &camera_query.single();

                    // println!("{:?}", cursor);
                    println!("{:?}", &camera_query.single());

                    commands.spawn(Block {
                        data: PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
                            transform: Transform::from_xyz(*state, 0.5, 0.0),
                            ..default()
                        },
                    });

                    *state += 1.0;
                }

                // println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                // println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
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
fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    // let window = windows.get_single().unwrap();
    // let mut cursor = window.cursor;

    if key.just_pressed(KeyCode::LControl) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        // cursor.set_cursor_grab_mode(CursorGrabMode::Confined);

        println!("{:?}", PrimaryWindow);
        println!("locking cursor");
    }

    if key.just_pressed(KeyCode::Escape) {
        // window.set_cursor_grab_mode(CursorGrabMode::None);
        // window.set_cursor_visibility(true);
    }
}
