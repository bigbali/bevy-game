use bevy::{prelude::*, utils::HashMap};

use block_mesh::ndshape::ConstShape3u32;

use super::block::{Block, BlockType};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_WITH_PADDING: u32 = CHUNK_SIZE as u32 + 2;

#[derive(Resource, Default, Debug, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkRegistry {
    pub chunks: HashMap<IVec3, Chunk>,
}

#[derive(Default, Debug, Reflect, FromReflect)]
pub struct Chunk {
    // possibly more sensible to use hashmap in order to access block by coords?
    #[reflect(ignore)]
    pub blocks: Vec<Block>,
    pub position: IVec3,
}

pub type ChunkShape =
    ConstShape3u32<CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING, CHUNK_SIZE_WITH_PADDING>;
