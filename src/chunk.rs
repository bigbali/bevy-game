use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};
use bevy_aabb_instancing::{
    Cuboid, CuboidMaterial, CuboidMaterialId, CuboidMaterialMap, Cuboids, COLOR_MODE_SCALAR_HUE,
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use crate::{
    block::{BlockMaterialStore, BlockType},
    Block,
};

const CHUNK_SIZE: usize = 16;
const INITIAL_WORLD_SIZE_FOR_TESTING: usize = 16;
const HEIGHTMAP_SIZE: usize = CHUNK_SIZE * INITIAL_WORLD_SIZE_FOR_TESTING;

pub struct ChunkRegistry(HashMap<IVec3, Chunk>);

#[derive(Component)]
pub struct Chunk {
    // pub blocks: [Option<Block>; CHUNK_SIZE.pow(3)],
    pub blocks: Vec<Option<Block>>,
    pub position: IVec3,
}

impl Chunk {
    pub fn new(
        heightmap: &Vec<f64>,
        mut meshes: &ResMut<Assets<Mesh>>,
        mut commands: &mut Commands,
        mut material_map: &ResMut<CuboidMaterialMap>,
        block_materials: &ResMut<BlockMaterialStore>,
        asset_server: &Res<AssetServer>,
        materials: &Res<Assets<StandardMaterial>>,
        pos: Vec3,
    ) -> Self {
        let mut instances = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..64 {
                    // let height = (heightmap[x as usize * z as usize] as f32 * 4.0 + 8.0).round();
                    let height = (heightmap[((pos.x as i32 * CHUNK_SIZE as i32 + x as i32)
                        * HEIGHTMAP_SIZE as i32
                        + pos.z as i32 * CHUNK_SIZE as i32
                        + z as i32) as usize] as f32
                        * CHUNK_SIZE as f32)
                        .round();

                    if y as f32 > height {
                        continue;
                    }

                    let color;
                    if height as i32 == y {
                        let material = block_materials.data.get(&BlockType::Grass).unwrap();
                        color = materials.get(material).unwrap().base_color;
                    } else {
                        let material = block_materials.data.get(&BlockType::Stone).unwrap();
                        color = materials.get(material).unwrap().base_color;
                    };

                    let x = x as f32 + (pos.x * CHUNK_SIZE as f32);
                    let z = z as f32 + (pos.z * CHUNK_SIZE as f32);

                    let mut cuboid = Cuboid::new(
                        Vec3::new(x, y as f32, z),
                        Vec3::new(x + 1.0, y as f32 + 1.0, z + 1.0),
                        color.as_rgba_u32(),
                    );
                    cuboid.set_depth_bias(0);

                    instances.push(cuboid);

                    // instances.push(TestBlock)
                }
            }
        }

        let cuboids = Cuboids::new(instances);
        let aabb = cuboids.aabb();
        commands
            .spawn(SpatialBundle::default())
            .insert((cuboids, aabb, CuboidMaterialId(0)));
        return Self {
            blocks: Vec::with_capacity(CHUNK_SIZE.pow(3)),
            position: IVec3::ZERO,
        };
    }

    pub fn get_block(&self, x: u8, y: u8, z: u8) -> Option<&Block> {
        let index = x as usize * CHUNK_SIZE * CHUNK_SIZE + y as usize * CHUNK_SIZE + z as usize;
        self.blocks.get(index).unwrap().as_ref()
    }

    pub fn get_some_noise() -> NoiseMap {
        let fbm = Fbm::<Perlin>::new(999);

        let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(HEIGHTMAP_SIZE, HEIGHTMAP_SIZE)
            .build();

        return noise;
    }
}

pub fn initialize_example_chunk(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    material_store: ResMut<BlockMaterialStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_map: ResMut<CuboidMaterialMap>,
    asset_server: Res<AssetServer>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let noise = Chunk::get_some_noise();

    // with this we generate the terrain
    let mut height_data = Vec::with_capacity(HEIGHTMAP_SIZE * HEIGHTMAP_SIZE);
    // and with this we render the heightmap to the UI
    let mut rgba_data = Vec::with_capacity(HEIGHTMAP_SIZE * HEIGHTMAP_SIZE * 4);

    for value in noise.iter() {
        let height = (value * 0.5 + 0.5).clamp(0.0, 1.0);

        height_data.push(height);

        let height_as_rgba = (height * 255.0) as u8;
        rgba_data.push(height_as_rgba);
        rgba_data.push(height_as_rgba);
        rgba_data.push(height_as_rgba);

        // alpha
        rgba_data.push(255);
    }

    for x in 0..INITIAL_WORLD_SIZE_FOR_TESTING {
        for z in 0..INITIAL_WORLD_SIZE_FOR_TESTING {
            Chunk::new(
                &height_data,
                &meshes,
                &mut commands,
                &material_map,
                &material_store,
                &asset_server,
                &materials,
                Vec3 {
                    x: x as f32,
                    y: 1.0,
                    z: z as f32,
                },
            );
        }
    }

    let image = images.add(Image::new(
        Extent3d {
            width: HEIGHTMAP_SIZE as u32,
            height: HEIGHTMAP_SIZE as u32,
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
