use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, Texture, TextureDimension, TextureFormat},
        renderer::RenderDevice,
    },
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use crate::Block;

const CHUNK_SIZE: usize = 32;

#[derive(Component)]
pub struct Chunk {
    pub blocks: [[[Option<Block>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    // pub blocks: Vec<Option<Block>>,
    pub position: IVec2,
}

impl Chunk {
    pub fn get_some_noise() -> NoiseMap {
        let fbm = Fbm::<Perlin>::new(999);

        let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(CHUNK_SIZE, CHUNK_SIZE)
            .build();
        // for x in noise.iter() {
        //     println!("{:?}", x);
        // }

        return noise;
    }
}

pub fn initialize_example_chunk(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let noise = Chunk::get_some_noise();

    // with this we generate the terrain
    let mut data = Vec::with_capacity(CHUNK_SIZE.pow(3));
    // and with this we render the heightmap to the UI
    let mut rgba_data = Vec::with_capacity(CHUNK_SIZE.pow(3));

    for value in noise.iter() {
        let uniform_rgb_channel = ((value * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;

        data.push(uniform_rgb_channel);

        rgba_data.push(uniform_rgb_channel);
        rgba_data.push(uniform_rgb_channel);
        rgba_data.push(uniform_rgb_channel);

        // alpha
        rgba_data.push(255);
    }

    let image = images.add(Image::new(
        Extent3d {
            width: CHUNK_SIZE as u32,
            height: CHUNK_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba_data,
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
