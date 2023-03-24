use bevy::{prelude::*, utils::HashMap};

const CHUNK_DIMENSIONS: u8 = 32;

struct Chunk {
    blocks: [[[Block; 32]; 32]; 32],
    position: GlobalTransform,
}
