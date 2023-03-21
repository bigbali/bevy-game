use bevy::prelude::*;

#[derive(Debug)]
pub enum BlockType {
    Stone,
    Soil,
    Grass,
    MIX,
}

#[derive(Default, Clone, Debug)]
pub enum VoxelType {
    #[default]
    Stone,
    Soil,
    Grass,
}

#[derive(Default, Clone, Debug)]
pub struct Voxel {
    position: Vec3,
    voxel_type: VoxelType,
}

#[derive(Debug)]
pub struct Block {
    position: Vec3,
    block_type: BlockType,
    voxel_data: Vec<Voxel>,
}

impl Block {
    pub fn create(block_type: BlockType) -> Block {
        match block_type {
            BlockType::Stone => StoneBlock::create(),
            _ => {
                println!(
                    "Function 'create' for block of type {:?} is not implemented.",
                    block_type
                );

                return StoneBlock::create();
            }
        }
    }
}

pub struct StoneBlock {}

fn fill_voxels() -> Vec<Voxel> {
    let mut vdata = Vec::with_capacity(32 * 32 * 32);
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                vdata.push(Voxel {
                    position: Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: z as f32,
                    },
                    voxel_type: VoxelType::Stone,
                });
            }
        }
    }

    return vdata;
}

impl StoneBlock {
    fn create() -> Block {
        let block = Block {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            block_type: BlockType::Stone,
            voxel_data: fill_voxels(),
        };

        println!("Created block {:?}", block);

        return block;
    }
}
