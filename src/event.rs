use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
};
use bevy_rapier3d::{prelude::*, render::ColliderDebugColor};

use crate::Block;

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockHighlightEvent>()
            .add_event::<BlockSpawnEvent>()
            .add_system(spawn_block)
            .add_system(block_highlight_send)
            .add_system(block_highlight_receive)
            .add_system(mouse_button_events)
            .insert_resource(BlockHighlightEventPrevious::default());
    }
}

struct BlockSpawnEvent {
    entity: Entity,
    position: Vec3,
    color: Color,
}

#[derive(Debug)]
struct BlockHighlightEvent {
    entity: Entity,
    intersection: RayIntersection,
}

#[derive(Resource, Default, Debug)]
struct BlockHighlightEventPrevious {
    entity: Option<Entity>,
    material: Option<Handle<StandardMaterial>>,
}

fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_block: EventReader<BlockSpawnEvent>,
    mut previously_highlighted: ResMut<BlockHighlightEventPrevious>,
) {
    for spawn in spawn_block.iter() {
        let color = spawn.color;
        let position = spawn.position;

        let mesh = Mesh::from(shape::Cube { size: 1.0 });
        let mesh_handle = meshes.add(mesh);

        let material = materials.add(StandardMaterial {
            base_color: color,
            ..Default::default()
        });

        let transform = Transform::from_translation(position);

        let block_entity = commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                material,
                transform,
                ..Default::default()
            })
            .insert(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(ColliderDebugColor(Color::VIOLET))
            .id();

        previously_highlighted.entity = Some(block_entity);
    }
}

fn block_highlight_receive(
    mut highlight_block: EventReader<BlockHighlightEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color: Query<&mut Handle<StandardMaterial>>,
    mut previously_highlighted: ResMut<BlockHighlightEventPrevious>,
) {
    for event in highlight_block.iter() {
        if let Some(entity) = previously_highlighted.entity {
            // reset the material for the previously highlighted block
            if let Some(mut prev_color) = color.get_mut(entity).ok() {
                *prev_color = previously_highlighted.material.as_mut().unwrap().clone();
                // println!("resetting prev")
            }

            if entity != event.entity {
                println!("aiming at new entity")
            }
        }

        // update the previously highlighted block to be the current one
        previously_highlighted.material = Some(color.get(event.entity).unwrap().clone());
        previously_highlighted.entity = Some(event.entity);

        if let Some(mut current_color) = color.get_mut(event.entity).ok() {
            *current_color = materials.add(Color::GREEN.into());
        }
    }
}

fn block_highlight_send(
    windows: Query<&mut Window>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    color: Query<&Handle<StandardMaterial>>,
    mut highlight_block: EventWriter<BlockHighlightEvent>,
    mut previously_highlighted: ResMut<BlockHighlightEventPrevious>,
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
        highlight_block.send(BlockHighlightEvent {
            entity,
            intersection,
        });
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
    mut writer: EventWriter<BlockSpawnEvent>,
    mut previously_highlighted: ResMut<BlockHighlightEventPrevious>,
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

                        let color = Color::GREEN;
                        commands.entity(entity).insert(ColliderDebugColor(color));

                        // MaterialMeshBundle
                        let mesh = Mesh::from(shape::Cube { size: 1.0 });
                        let block = meshes.add(mesh.clone());

                        writer.send(BlockSpawnEvent {
                            position: Vec3 {
                                x: hit_point.x + hit_normal.x,
                                y: hit_point.y + hit_normal.y,
                                z: hit_point.z + hit_normal.z,
                            },
                            color: Color::YELLOW,
                            entity,
                        });
                    }
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
            }
            ButtonState::Released => {}
        }
    }
}
