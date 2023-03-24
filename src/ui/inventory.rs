use bevy::prelude::*;

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
                            width: Val::Px(576.0), // 9 * 60 + 2 * 3 (3: container padding, 60: slot size, 9: slot count)
                            height: Val::Px(66.0), // 60 + 2 * 3
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
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            })
                            .with_children(|slot_rectangle| {
                                // slot numbering
                                slot_rectangle.spawn(
                                    TextBundle::from_section(
                                        format!("{}", slot),
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
                                            right: Val::Px(4.0),
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
