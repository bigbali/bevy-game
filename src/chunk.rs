use bevy::{prelude::*, utils::HashMap};

const CHUNK_DIMENSIONS: u8 = 32;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum BlockType {
    #[default]
    Stone,
    Soil,
    Grass,
    MIXED,
}

#[derive(Resource)]
pub struct BlockMaterial {
    data: HashMap<BlockType, StandardMaterial>,
}

impl BlockMaterial {
    pub fn new() -> Self {
        let mut materials = HashMap::new();
        materials.insert(
            BlockType::Stone,
            StandardMaterial {
                base_color: Color::DARK_GRAY,
                ..default()
            },
        );
        materials.insert(
            BlockType::Soil,
            StandardMaterial {
                base_color: Color::MAROON,
                ..default()
            },
        );
        materials.insert(
            BlockType::Grass,
            StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            },
        );
        materials.insert(
            BlockType::MIXED,
            StandardMaterial {
                base_color: Color::GOLD,
                ..default()
            },
        );
        return BlockMaterial { data: materials };
    }

    pub fn get_material(&self, block_type: BlockType) -> Option<&StandardMaterial> {
        self.data.get(&block_type)
    }
}

struct Block {
    render: PbrBundle,
    position_in_chunk: Vec3,
    block_type: BlockType,
}

impl Block {
    pub fn create(
        &self,
        block_type: BlockType,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) -> Option<Block> {
        match block_type {
            BlockType::MIXED => {
                println!(
                    "Function 'create' for block of type {:?} is not implemented.",
                    block_type
                );

                return None;
            }
            _ => {
                return None;
                // return Block::;
            }
        }
    }
}

struct Chunk {
    blocks: [[[Block; 32]; 32]; 32],
    position: GlobalTransform,
}
