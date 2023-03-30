use std::array;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::HashMap,
};
use bevy_aabb_instancing::{
    Cuboid, CuboidMaterial, CuboidMaterialId, CuboidMaterialMap, Cuboids, COLOR_MODE_SCALAR_HUE,
};
use block_mesh::ndshape::ConstShape3u32;

use super::block::Block;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_WITH_PADDING: u32 = CHUNK_SIZE as u32 + 2;

pub struct ChunkRegistry(HashMap<IVec3, Chunk>);

#[derive(Component)]
pub struct Chunk {
    pub blocks: Vec<Option<Block>>,
    pub position: IVec3,
}

pub type ChunkShape =
    ConstShape3u32<CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING>;
