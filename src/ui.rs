use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(initialize_fps_counter_system)
            .add_startup_system(initialize_crosshair)
            .add_system(ui_update_system);
    }
}

#[derive(Component)]
struct FpsText;
fn initialize_fps_counter_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "FPS [default]",
            TextStyle {
                font: asset_server.load("font/TiltWarp-Regular.ttf"),
                font_size: 35.0,
                color: Color::GREEN,
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
        FpsText,
    ));
}

fn initialize_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ImageBundle {
        image: UiImage {
            texture: asset_server.load("image/crosshair.png"),
            ..default()
        },
        style: Style {
            margin: UiRect::all(Val::Auto),
            size: Size {
                width: Val::Px(5.0),
                height: Val::Px(5.0),
            },
            ..default()
        },
        ..default()
    });
}

fn ui_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("{value:.0}");
            }
        }
    }
}
