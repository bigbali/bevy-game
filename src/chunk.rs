use std::array;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};
use bevy_aabb_instancing::{
    Cuboid, CuboidMaterial, CuboidMaterialId, CuboidMaterialMap, Cuboids, COLOR_MODE_SCALAR_HUE,
};
use block_mesh::{
    greedy_quads,
    ilattice::prelude::Vec3A,
    ndshape::{ConstShape, ConstShape3u32},
    visible_block_faces, GreedyQuadsBuffer, MergeVoxel, OrientedBlockFace, UnitQuadBuffer, Voxel,
    VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use crate::{
    block::{BlockMaterialStore, BlockType},
    Block,
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_WITH_PADDING: u32 = CHUNK_SIZE as u32 + 2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BlockXXX {
    position: UVec3,
    visible: bool,
    block_type: BlockType,
}

impl Voxel for BlockXXX {
    fn get_visibility(&self) -> VoxelVisibility {
        // if *self == EMPTY {
        //     VoxelVisibility::Empty
        // } else {
        //     VoxelVisibility::Opaque
        // }
        VoxelVisibility::Opaque
    }
}

impl MergeVoxel for BlockXXX {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        *self
    }
}

pub struct ChunkRegistry(HashMap<IVec3, Chunk>);
// type ChunkShape =
//     ConstShape3u32<CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING>;

#[derive(Component)]
pub struct Chunk {
    // pub blocks: [Option<Block>; CHUNK_SIZE.pow(3)],
    pub blocks: Vec<Option<BlockXXX>>,
    pub position: IVec3,
}

type ChunkShape = ConstShape3u32<18, 18, 18>;

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
    ) -> () {
        // let mut instances = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
        let mut samples = Vec::with_capacity(ChunkShape::SIZE as usize);
        // let mut chunks: HashMap<[i32; 3], Vec<BlockXXX>> = HashMap::new();

        for x in 0..CHUNK_SIZE_WITH_PADDING {
            for z in 0..CHUNK_SIZE_WITH_PADDING {
                for y in 0..CHUNK_SIZE_WITH_PADDING {
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

                    if height as u32 == y {
                        samples.push(BlockXXX {
                            block_type: BlockType::Grass,
                            position: UVec3::new(x, y, z),
                            visible: true,
                        })
                    } else {
                        samples.push(BlockXXX {
                            block_type: BlockType::Stone,
                            position: UVec3::new(x, y, z),
                            visible: true,
                        })
                    };
                }
            }
        }

        let simple_mesh = generate_simple_mesh(&samples);

        // chunks.insert([pos.x as i32, pos.y as i32, pos.z as i32], samples);
    }

    pub fn get_some_noise() -> NoiseMap {
        let fbm = Fbm::<Perlin>::new(999);

        let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(HEIGHTMAP_SIZE, HEIGHTMAP_SIZE)
            .build();

        return noise;
    }
}
