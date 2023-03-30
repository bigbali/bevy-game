use bevy::prelude::*;

use crate::{
    event::SelectBlockEvent,
    world::block::{BlockMaterialStore, BlockType},
};

const INVENTORY_OVERLAY_SLOTS: usize = 9;
const SLOT_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const SLOT_SELECTED_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub struct InventorySystemPlugin;

impl Plugin for InventorySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_inventory_overlay)
            .add_system(update_inventory_overlay)
            .add_system(select_block)
            .add_event::<SelectInventorySlotEvent>()
            .insert_resource(Inventory::new())
            .insert_resource(SelectedBlock(BlockType::Stone))
            .register_type::<Inventory>()
            .register_type::<Slot>();
    }
}

#[derive(Component)]
struct InventorySlotComponent;

#[derive(Resource, Debug, Reflect, FromReflect, Clone)]
pub struct Slot {
    contains: Option<Color>,
}

impl Slot {
    pub fn new() -> Self {
        return Self { contains: None };
    }
}

#[derive(Resource, Debug, Reflect, FromReflect, Default)]
#[reflect(Resource)]
pub struct Inventory {
    pub items: Vec<Slot>,
    pub selected: usize,
}

impl Inventory {
    pub fn new() -> Self {
        return Self {
            items: vec![Slot::new(); INVENTORY_OVERLAY_SLOTS],
            selected: 0,
        };
    }

    pub fn get_selected(&mut self) -> &mut Slot {
        return &mut self.items[self.selected];
    }

    pub fn get_selected_with_index(&mut self) -> (usize, &mut Slot) {
        return (self.selected, &mut self.items[self.selected]);
    }
}

#[derive(Resource, Default, Debug)]
pub struct SelectedBlock(pub BlockType);

pub fn select_block(
    mut select_block: EventReader<SelectBlockEvent>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    for select_block_event in select_block.iter() {
        selected_block.0 = select_block_event.0;
        println!("Selected block: {:?}", selected_block);
    }
}

#[derive(Debug)]
pub struct SelectInventorySlotEvent(pub usize);

fn initialize_inventory_overlay(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    block_materials: ResMut<BlockMaterialStore>,
    asset_server: Res<AssetServer>,
    materials: Res<Assets<StandardMaterial>>,
) {
    commands
        // container
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        // actual UI
        .with_children(|container| {
            container
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(INVENTORY_OVERLAY_SLOTS as f32 * 63.0), // 60 + padding
                            height: Val::Px(66.0),
                        },
                        padding: UiRect::all(Val::Px(3.0)),
                        margin: UiRect {
                            bottom: Val::Px(3.0),
                            top: Val::Auto,
                            ..default()
                        }, // looks like alignment is off by a pixel or two
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|overlay| {
                    for slot in 0..INVENTORY_OVERLAY_SLOTS {
                        let maybe_color = match slot {
                            0 => Some(block_materials.get_color(BlockType::Stone, &materials)),
                            1 => Some(block_materials.get_color(BlockType::Soil, &materials)),
                            2 => Some(block_materials.get_color(BlockType::Grass, &materials)),
                            _ => None,
                        };

                        let mut display_color = Color::rgba(0.0, 0.0, 0.0, 0.0);

                        if let Some(block_color) = maybe_color {
                            let color = block_color;
                            inventory.items[slot].contains = Some(color);
                            display_color = color;
                        }

                        overlay
                            // slot rectangle
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::all(Val::Px(60.0)),
                                    margin: UiRect::vertical(Val::Auto),
                                    ..default()
                                },
                                background_color: match slot {
                                    0 => SLOT_SELECTED_COLOR.into(),
                                    _ => SLOT_COLOR.into(),
                                },
                                ..default()
                            })
                            .with_children(|slot_rectangle| {
                                slot_rectangle.spawn(NodeBundle {
                                    style: Style {
                                        margin: UiRect::all(Val::Auto),
                                        size: Size::all(Val::Px(35.0)),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: BackgroundColor::from(display_color),
                                    ..default()
                                });

                                // slot numbering
                                slot_rectangle.spawn(
                                    TextBundle::from_section(
                                        format!("{}", slot + 1),
                                        TextStyle {
                                            font: asset_server.load("font/TiltWarp-Regular.ttf"),
                                            font_size: 14.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center)
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            bottom: Val::Px(2.0),
                                            right: Val::Px(6.0),
                                            ..default()
                                        },
                                        ..default()
                                    }),
                                );
                            })
                            .insert(InventorySlotComponent);
                    }

                    println!("{:?}", inventory);
                });
        });
}

fn update_inventory_overlay(
    mut query: Query<&mut BackgroundColor, With<InventorySlotComponent>>,
    mut selected_inventory_slot: EventReader<SelectInventorySlotEvent>,
) {
    for event in selected_inventory_slot.iter() {
        let selected_slot = event.0;

        for (index, mut background_color) in query.iter_mut().enumerate() {
            if index == selected_slot {
                background_color.0 = SLOT_SELECTED_COLOR;
            } else {
                background_color.0 = SLOT_COLOR;
            }
        }
    }
}
