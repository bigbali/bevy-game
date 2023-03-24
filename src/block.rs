use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::Collider;

use crate::event::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .add_system(select_block)
            .insert_resource(SelectedBlock(BlockType::Stone));
    }
}

fn setup_resources(mut commands: Commands, materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(BlockMaterialStore::new(materials))
}

pub fn select_block(
    mut select_block: EventReader<SelectBlockEvent>,
    mut selected_block: ResMut<SelectedBlock>,
) {
    for select_block_event in select_block.iter() {
        selected_block.0 = select_block_event.0;
        println!("event: {:?}", select_block_event);
        println!("selected: {:?}", selected_block);
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Component, Copy, Clone)]
pub enum BlockType {
    #[default]
    Stone,
    Soil,
    Grass,
    MIXED,
}

#[derive(Resource, Default, Debug)]
pub struct SelectedBlock(pub BlockType);

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
pub struct Block {
    render: PbrBundle,
    // position_in_chunk: BlockPositionInChunk,
    block_type: BlockType,
    collider: Collider,
}

impl Block {
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
                    .spawn(Block {
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
