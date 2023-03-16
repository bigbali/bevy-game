use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

mod camera;
mod ui;

fn main() {
    println!("Application initializing.");

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraControllerPlugin)
        .add_plugin(ui::UserInterfacePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(print_resources)
        .add_startup_system(setup_physics)
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
    render: PbrBundle,
    collider: Collider,
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
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left {
                    // println!("Click.");

                    let (camera, camera_transform) = camera_query.single();

                    let window = windows.get_single().unwrap();
                    let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

                    // use this for cursor position instead of viewport center
                    // let cursor = window.cursor_position().unwrap();

                    let ray = camera
                        .viewport_to_world(camera_transform, window_center)
                        .unwrap();

                    if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
                        ray.origin,
                        ray.direction,
                        f32::MAX,
                        true,
                        QueryFilter::new(),
                    ) {
                        // This is similar to `QueryPipeline::cast_ray` illustrated above except
                        // that it also returns the normal of the collider shape at the hit point.
                        let hit_point = intersection.point;
                        let hit_normal = intersection.normal;
                        println!(
                            "Entity {:?} hit at point {} with normal {}",
                            entity, hit_point, hit_normal
                        );

                        let color = Color::BLUE;
                        commands.entity(entity).insert(ColliderDebugColor(color));
                    }

                    // println!("{:?}", ray);

                    let mesh = Mesh::from(shape::Cube { size: 1.0 });
                    let block = meshes.add(mesh.clone());

                    commands.spawn(Block {
                        render: PbrBundle {
                            mesh: block,
                            material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
                            transform: Transform::from_xyz(*state, 0.5, 0.0),
                            ..default()
                        },
                        collider: Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
                            .unwrap(),
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

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    /*
     * Create the cubes
     */
    // let num = 8;
    // let rad = 1.0;

    // let shift = rad * 2.0 + rad;
    // let centerx = shift * (num / 2) as f32;
    // let centery = shift / 2.0;
    // let centerz = shift * (num / 2) as f32;

    // let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    // for j in 0usize..20 {
    //     for i in 0..num {
    //         for k in 0usize..num {
    //             let x = i as f32 * shift - centerx + offset;
    //             let y = j as f32 * shift + centery + 3.0;
    //             let z = k as f32 * shift - centerz + offset;

    //             // Build the rigid body.
    //             commands.spawn((
    //                 TransformBundle::from(Transform::from_xyz(x, y, z)),
    //                 RigidBody::Dynamic,
    //                 Collider::cuboid(rad, rad, rad),
    //             ));
    //         }
    //     }

    //     offset -= 0.05 * rad * (num as f32 - 1.0);
    // }
}
