use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransform, LookTransformPlugin,
};

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin)
            .add_plugin(FpsCameraPlugin::default())
            .add_startup_system(create_camera_system)
            .add_system(move_camera_system);
    }
}

fn create_camera_system(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 1.0,
                ..Default::default()
            },
            Vec3::new(10.0, 10.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ));
}

fn move_camera_system(mut cameras: Query<&mut LookTransform>) {
    // Later, another system will update the `Transform` and apply smoothing automatically.
    for mut c in cameras.iter_mut() {
        c.target += Vec3::new(0.0, 0.0, 0.0);
    }
}
