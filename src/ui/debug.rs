use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// Show the heightmap from which we are generating our world.
pub fn initialize_heightmap_overlay(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    width: u32,
    height: u32,
    data: Vec<u8>,
) {
    let image = images.add(Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
    ));
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::all(Val::Px(200.0)),
                position: UiRect {
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|container| {
            container.spawn(ImageBundle {
                image: UiImage::new(image),
                ..default()
            });
        });
}
