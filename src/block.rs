use bevy::prelude::*;

// #[derive(Debug)]
// pub enum BlockType {
//     Stone,
//     Soil,
//     Grass,
//     MIX,
// }

// #[derive(Default, Clone, Debug)]
// pub enum VoxelType {
//     #[default]
//     Stone,
//     Soil,
//     Grass,
// }

// #[derive(Default, Clone, Debug)]
// pub struct Voxel {
//     position: Vec3,
//     voxel_type: VoxelType,
// }

// #[derive(Debug)]
// pub struct Block {
//     position: Vec3,
//     block_type: BlockType,
//     voxel_data: Vec<Voxel>,
// }

// impl Block {
//     pub fn create(block_type: BlockType) -> Block {
//         match block_type {
//             BlockType::Stone => StoneBlock::create(),
//             _ => {
//                 println!(
//                     "Function 'create' for block of type {:?} is not implemented.",
//                     block_type
//                 );

//                 return StoneBlock::create();
//             }
//         }
//     }
// }

// pub struct StoneBlock {}

// fn fill_voxels() -> Vec<Voxel> {
//     let mut vdata = Vec::with_capacity(32 * 32 * 32);
//     for x in 0..32 {
//         for y in 0..32 {
//             for z in 0..32 {
//                 vdata.push(Voxel {
//                     position: Vec3 {
//                         x: x as f32,
//                         y: y as f32,
//                         z: z as f32,
//                     },
//                     voxel_type: VoxelType::Stone,
//                 });
//             }
//         }
//     }

//     return vdata;
// }

// impl StoneBlock {
//     fn create() -> Block {
//         let block = Block {
//             position: Vec3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//             },
//             block_type: BlockType::Stone,
//             voxel_data: fill_voxels(),
//         };

//         println!("Created block {:?}", block);

//         return block;
//     }
// }

use bevy::prelude::StandardMaterial;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::Collider;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources);
    }
}

fn setup_resources(mut commands: Commands, materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(BlockMaterialStore::new(materials))
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Component)]
pub enum BlockType {
    #[default]
    Stone,
    Soil,
    Grass,
    MIXED,
}

// #[derive(Resource)]
// pub struct BlockMesh(Handle<Mesh>);

// impl BlockMesh {
//     pub fn new(mut meshes_resource: ResMut<Assets<Mesh>>) -> Handle<Mesh> {
//         meshes_resource.add(Mesh::from(shape::Cube { size: 1.0 }))
//     }
// }

#[derive(Resource)]
pub struct BlockMaterialStore {
    data: HashMap<BlockType, Handle<StandardMaterial>>,
}

impl BlockMaterialStore {
    pub fn new(mut materials_resource: ResMut<Assets<StandardMaterial>>) -> Self {
        let mut materials = HashMap::new();
        materials.insert(
            BlockType::Stone,
            materials_resource.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                ..default()
            }),
        );
        materials.insert(
            BlockType::Soil,
            materials_resource.add(StandardMaterial {
                base_color: Color::MAROON,
                ..default()
            }),
        );
        materials.insert(
            BlockType::Grass,
            materials_resource.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
        );
        materials.insert(
            BlockType::MIXED,
            materials_resource.add(StandardMaterial {
                base_color: Color::GOLD,
                ..default()
            }),
        );

        return BlockMaterialStore { data: materials };
    }

    pub fn get_material(&self, block_type: BlockType) -> Option<&Handle<StandardMaterial>> {
        self.data.get(&block_type)
    }
}

#[derive(Component)]
pub struct BlockPositionInChunk(Vec3);

#[derive(Bundle)]
pub struct BlockTest {
    render: PbrBundle,
    // position_in_chunk: BlockPositionInChunk,
    block_type: BlockType,
    collider: Collider,
}

impl BlockTest {
    pub fn create(
        block_type: BlockType,
        material_store: &ResMut<BlockMaterialStore>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        position: Vec3,
    ) -> Option<Entity> {
        match block_type {
            BlockType::MIXED => {
                println!(
                    "Function 'create' for block of type {:?} is not implemented.",
                    block_type
                );

                return None;
            }
            _ => {
                let material = material_store.data.get(&block_type).unwrap();

                let entity = commands
                    .spawn(BlockTest {
                        block_type,
                        render: PbrBundle {
                            material: material.clone(),
                            transform: Transform::from_translation(position),
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            ..default()
                        },
                        collider: Collider::cuboid(0.5, 0.5, 0.5),
                    })
                    .id();

                println!("Creating block! {:?}", material);
                return Some(entity);
            }
        }
    }
}
