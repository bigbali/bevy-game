use bevy_mod_outline::*;

use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState,
    },
    prelude::*,
};
use bevy_rapier3d::{prelude::*, render::ColliderDebugColor};

use crate::{
    ui::inventory::{SelectInventorySlotEvent, SelectedBlock},
    world::block::*,
};

// use crate::{block::*, ui::inventory::SelectInventorySlotEvent};

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HighlightBlock>()
            .add_event::<RemoveBlockHighlight>()
            .add_event::<BlockSpawnEvent>()
            .add_event::<SelectBlockEvent>()
            .add_plugin(OutlinePlugin)
            .insert_resource(Msaa::Sample4)
            .add_system(spawn_block)
            .add_system(select_block_to_spawn)
            .add_system(highlight_block_at_crosshair)
            .add_system(highlight_block)
            .add_system(mouse_button_events)
            .add_system(spawn_block.run_if(resource_exists::<BlockMaterialStore>()))
            .insert_resource(BlockHighlightEventPrevious::default());
    }
}

#[derive(Debug)]
pub struct SelectBlockEvent(pub BlockType);

#[allow(dead_code)]
pub struct BlockSpawnEvent {
    entity: Entity,
    position: Vec3,
    color: Color,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HighlightBlock {
    entity: Entity,
    intersection: RayIntersection,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RemoveBlockHighlight;

#[derive(Resource, Default, Debug)]
struct BlockHighlightEventPrevious {
    entity: Option<Entity>,
    material: Option<Handle<StandardMaterial>>,
}

#[derive(Resource, Default, Debug)]
struct HighlightedBlock {
    entity: Option<Entity>,
}

// TODO some blocks might prefer to stay in the shadows!
#[derive(Component)]
pub struct Highlightable;

fn select_block_to_spawn(
    mut input: EventReader<KeyboardInput>,
    mut select_block: EventWriter<SelectBlockEvent>,
    mut select_inv: EventWriter<SelectInventorySlotEvent>,
) {
    for event in input.iter() {
        match event.state {
            ButtonState::Pressed => match event.key_code {
                Some(code) => match code {
                    KeyCode::Key1 => {
                        select_block.send(SelectBlockEvent(BlockType::Stone));
                        select_inv.send(SelectInventorySlotEvent(0));
                    }
                    KeyCode::Key2 => {
                        select_block.send(SelectBlockEvent(BlockType::Soil));
                        select_inv.send(SelectInventorySlotEvent(1));
                    }
                    KeyCode::Key3 => {
                        select_block.send(SelectBlockEvent(BlockType::Grass));
                        select_inv.send(SelectInventorySlotEvent(2));
                    }
                    _ => {}
                },
                None => {}
            },
            ButtonState::Released => {}
        }
    }
}

fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawn_block: EventReader<BlockSpawnEvent>,
    material_store: ResMut<BlockMaterialStore>,
    selected_block: Res<SelectedBlock>,
) {
    for spawn in spawn_block.iter() {
        let position = spawn.position;

        println!("EVENT: Spawn Block");

        // Block::create(
        //     selected_block.0,
        //     &material_store,
        //     &mut commands,
        //     &mut meshes,
        //     position,
        // );
    }

    spawn_block.clear();
}

// TODO surely we can do better
fn highlight_block(
    mut remove_block_highlight: EventReader<RemoveBlockHighlight>,
    mut highlight_block: EventReader<HighlightBlock>,
    mut previously_highlighted: ResMut<BlockHighlightEventPrevious>,
    mut commands: Commands,
) {
    for _ in remove_block_highlight.iter() {
        if let Some(previous_entity) = previously_highlighted.entity {
            commands.entity(previous_entity).remove::<OutlineBundle>();
        }
    }

    for highlight_event in highlight_block.iter() {
        if let Some(previous_entity) = previously_highlighted.entity {
            commands.entity(previous_entity).remove::<OutlineBundle>();
        }

        commands
            .entity(highlight_event.entity)
            .remove::<OutlineBundle>();

        commands
            .entity(highlight_event.entity)
            .insert(OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    colour: Color::rgba(0.8, 0.4, 0.8, 1.0),
                    width: 2.0,
                },
                ..default()
            });

        previously_highlighted.entity = Some(highlight_event.entity);
    }
}

/// Triggers a BlockHighlightEvent.
pub fn highlight_block_at_crosshair(
    windows: Query<&mut Window>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut highlight_block: EventWriter<HighlightBlock>,
    mut remove_block_highlight: EventWriter<RemoveBlockHighlight>,
    _: EventReader<MouseMotion>,
) {
    // let (camera, camera_transform) = camera_query.single();

    // let window = windows.get_single().unwrap();
    // let window_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // let ray = camera
    //     .viewport_to_world(camera_transform, window_center)
    //     .unwrap();

    // if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
    //     ray.origin,
    //     ray.direction,
    //     f32::MAX,
    //     true,
    //     QueryFilter::new(),
    // ) {
    //     highlight_block.send(HighlightBlock {
    //         entity,
    //         intersection,
    //     });
    // } else {
    //     remove_block_highlight.send(RemoveBlockHighlight);
    // }
}

fn mouse_button_events(
    windows: Query<&mut Window>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut commands: Commands,
    mut mouse_button: EventReader<MouseButtonInput>,
    mut block_spawn: EventWriter<BlockSpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    material_store: ResMut<BlockMaterialStore>,
) {
    for ev in mouse_button.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left {
                    let (camera, camera_transform) = camera_query.single();
                    println!("SENDING EVENT");

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

                        commands
                            .entity(entity)
                            .insert(ColliderDebugColor(Color::GREEN));

                        block_spawn.send(BlockSpawnEvent {
                            // while flooring the value guarantees axis-aligned placement, it does not necessarily guarantee correct axis :]
                            position: (hit_point + hit_normal).round(),
                            color: Color::YELLOW,
                            entity,
                        });
                    }
                }

                if ev.button == MouseButton::Right {
                    println!("Disabled: Generate Initial Block Entities");
                    // for x in -5..5 {
                    //     for y in -5..5 {
                    //         for z in -5..5 {
                    //             Block::create(
                    //                 BlockType::Stone,
                    //                 &material_store,
                    //                 &mut commands,
                    //                 &mut meshes,
                    //                 Vec3 {
                    //                     x: x as f32,
                    //                     y: y as f32,
                    //                     z: z as f32,
                    //                 },
                    //             );
                    //         }
                    //     }
                    // }
                }
            }
            ButtonState::Released => {}
        }
    }
}
