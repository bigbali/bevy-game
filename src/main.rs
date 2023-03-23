use std::f32::consts::PI;

use bevy::{
    ecs::system::EntityCommand,
    input::mouse::{MouseButtonInput, MouseMotion},
    pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_rapier3d::prelude::*;

use crate::block::BlockPlugin;

mod block;
mod camera;
// mod chunk;
mod event;
mod ui;
mod util;

fn main() {
    println!("Application initializing.");

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
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
        .add_startup_system(setup)
        .add_startup_system(print_resources)
        // .add_system(mouse_button_events)
        // .add_system(block_highlight_system)
        .add_system(cursor_grab_system)
        .add_system(fixed.in_schedule(CoreSchedule::FixedUpdate))
        // .add_system(build_voxel_block)
        .add_event::<SB>()
        .add_event::<BlockHighlightEvent>()
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
}

// #[derive(Component)]
// struct BlockComponent {
//     position: Vec3,
//     color: Color,
//     mesh: Mesh,
//     collider: Collider,
// }

#[derive(Bundle)]
struct Block {
    render: PbrBundle,
    collider: Collider,
}

fn fixed() {
    println!("running fixed system")
}

struct SB {
    position: Vec3,
    color: Color,
}

struct BlockHighlightEvent {
    entity: Entity,
    intersection: RayIntersection,
}

fn build_voxel_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_block: EventReader<SB>,
) {
    for spawn in spawn_block.iter() {
        println!("{:?} {:?}", spawn.color, spawn.position);

        let color = spawn.color;
        let position = spawn.position;

        let mesh = Mesh::from(shape::Cube { size: 1.0 });
        let mesh_handle = meshes.add(mesh);

        let material = materials.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });

        let transform = Transform::from_translation(position);

        // commands.spawn(VoxelBlock { fu: material. });

        let cube_entity = commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                material,
                transform,
                ..Default::default()
            })
            .insert(Collider::cuboid(0.5, 0.5, 0.5));
        // .insert(ColliderDebugColor(Color::VIOLET));
    }
}

fn block_highlight_system(
    windows: Query<&mut Window>,
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color: Query<&mut Handle<StandardMaterial>>,
    xxx: Query<(Entity)>,
    mut highlight_block: EventWriter<BlockHighlightEvent>,
    _: EventReader<MouseMotion>,
) {
    let (camera, camera_transform) = camera_query.single();

    let window = windows.get_single().unwrap();
    let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

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
        // let mut x = color.get_mut(entity).unwrap();
        // println!("{:?}", x);

        highlight_block.send(BlockHighlightEvent {
            entity,
            intersection,
        });

        *color.get_mut(entity).unwrap() = materials.add(Color::BEIGE.into());
        // let xx = xxx.get_mut(entity).unwrap();
        // xx.
        // let hit_point = intersection.point;
        // let hit_normal = intersection.normal;

        // commands.entity(entity).

        // let color = Color::BLUE;
        // let mesh = Mesh::from(shape::Cube { size: 1.0 });
        // let block = meshes.add(mesh.clone());

        // commands.entity(entity).insert(Block {
        //     render: PbrBundle {
        //         mesh: block,
        //         material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
        //         transform: Transform::from_xyz(
        //             hit_point.x + hit_normal.x,
        //             hit_point.y + hit_normal.y,
        //             hit_point.z + hit_normal.z,
        //         ),
        //         ..default()
        //     },
        //     collider: Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
        // });

        // let (x,y,z) = query_blocks.get_mut(entity).unwrap();
        // z.

        // let mut x = world.get_entity_mut(entity).unwrap();
        // x.insert(ColliderDebugColor(Color::VIOLET));

        // materials.get_mut(entity);
    }
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
    mut writer: EventWriter<SB>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left {
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
                        let hit_point = intersection.point;
                        let hit_normal = intersection.normal;
                        println!("Entity {:?} hit at point {:?}", entity, intersection);

                        let color = Color::GREEN;
                        commands.entity(entity).insert(ColliderDebugColor(color));

                        // MaterialMeshBundle
                        let mesh = Mesh::from(shape::Cube { size: 1.0 });
                        let block = meshes.add(mesh.clone());

                        writer.send(SB {
                            position: Vec3 {
                                x: hit_point.x + hit_normal.x,
                                y: hit_point.y + hit_normal.y,
                                z: hit_point.z + hit_normal.z,
                            },
                            color: Color::YELLOW,
                        });

                        // commands.spawn(Block {
                        //     render: PbrBundle {
                        //         mesh: block,
                        //         material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
                        //         transform: Transform::from_xyz(
                        //             hit_point.x + hit_normal.x,
                        //             hit_point.y + hit_normal.y,
                        //             hit_point.z + hit_normal.z,
                        //         ),
                        //         ..default()
                        //     },
                        //     collider: Collider::from_bevy_mesh(
                        //         &mesh,
                        //         &ComputedColliderShape::TriMesh,
                        //     )
                        //     .unwrap(),
                        // });
                    }

                    // block::Block::create(block::BlockType::Stone);

                    *state += 1.0;
                }
                if ev.button == MouseButton::Right {
                    for x in -5..5 {
                        for y in -5..5 {
                            for z in -5..5 {
                                let mesh = Mesh::from(shape::Cube { size: 1.0 });
                                let block = meshes.add(mesh.clone());
                                commands.spawn(Block {
                                    render: PbrBundle {
                                        mesh: block,
                                        material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
                                        transform: Transform::from_xyz(
                                            x as f32, y as f32, z as f32,
                                        ),
                                        ..default()
                                    },
                                    collider: Collider::from_bevy_mesh(
                                        &mesh,
                                        &ComputedColliderShape::TriMesh,
                                    )
                                    .unwrap(),
                                });
                            }
                        }
                    }
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
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut window = windows.get_single_mut().unwrap();

    if key.just_pressed(KeyCode::LControl) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;

        // let mesh = Mesh::from(shape::Cube { size: 1.0 });
        // let block = meshes.add(mesh.clone());

        // commands.spawn(Block {
        //     render: PbrBundle {
        //         mesh: block,
        //         material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
        //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //         ..default()
        //     },
        //     collider: Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
        // });

        println!("locking cursor");
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
