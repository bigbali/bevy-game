use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin)
            .add_plugin(FpsCameraPlugin::default())
            .add_startup_system(create_camera_system);
    }
}

fn create_camera_system(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 5.0,
                mouse_rotate_sensitivity: Vec2 { x: 0.15, y: 0.15 },
                smoothing_weight: 0.0,
                ..default()
            },
            Vec3::new(5.0, 5.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ));
}
