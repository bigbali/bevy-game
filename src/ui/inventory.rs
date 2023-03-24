use bevy::{prelude::*, ui::*};

const INVENTORY_OVERLAY_SLOTS: u8 = 9;

pub fn initialize_inventory_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // main container
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        // actual graphical interface
        .with_children(|screen| {
            screen
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Px(600.0),
                            height: Val::Px(100.0),
                        },
                        padding: UiRect::all(Val::Px(3.0)),
                        margin: UiRect {
                            bottom: Val::Px(3.0),
                            top: Val::Auto,
                            ..default()
                        },
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|overlay| {
                    // number the slots 1-9
                    for slot in 1..INVENTORY_OVERLAY_SLOTS + 1 {
                        overlay
                            // slot rectangle
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::all(Val::Px(60.0)),
                                    margin: UiRect::vertical(Val::Auto),
                                    ..default()
                                },
                                background_color: Color::GOLD.into(),
                                ..default()
                            })
                            .with_children(|slot_rectangle| {
                                // slot numbering
                                slot_rectangle.spawn(
                                    TextBundle::from_section(
                                        format!("{}", slot),
                                        TextStyle {
                                            font: asset_server.load("font/TiltWarp-Regular.ttf"),
                                            font_size: 8.0,
                                            color: Color::BLACK,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::Center)
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            top: Val::Px(1.0),
                                            left: Val::Px(1.0),
                                            ..default()
                                        },
                                        ..default()
                                    }),
                                );
                            });
                    }
                });
        });
}
