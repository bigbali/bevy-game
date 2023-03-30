use bevy::prelude::*;
use bevy::utils::HashMap;
use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockMaterialStore>()
            .insert_resource(BlockType::default());
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Block {
    position: UVec3,
    visible: bool,
    block_type: BlockType,
}

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        // if *self == EMPTY {
        //     VoxelVisibility::Empty
        // } else {
        //     VoxelVisibility::Opaque
        // }
        VoxelVisibility::Opaque
    }
}

impl MergeVoxel for Block {
    type MergeValue = Self;
    type MergeValueFacingNeighbour = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        *self
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Copy, Clone, Resource, Component)]
pub enum BlockType {
    #[default]
    Stone,
    Soil,
    Grass,
    MIXED,
}

#[derive(Resource, Debug)]
pub struct BlockMaterialStore {
    pub materials: HashMap<BlockType, Handle<StandardMaterial>>,
}

impl FromWorld for BlockMaterialStore {
    fn from_world(world: &mut World) -> Self {
        let mut standard_material_assets = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let mut materials = HashMap::new();
        materials.insert(
            BlockType::Stone,
            standard_material_assets.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                ..default()
            }),
        );
        materials.insert(
            BlockType::Soil,
            standard_material_assets.add(StandardMaterial {
                base_color: Color::MAROON,
                ..default()
            }),
        );
        materials.insert(
            BlockType::Grass,
            standard_material_assets.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
        );
        materials.insert(
            BlockType::MIXED,
            standard_material_assets.add(StandardMaterial {
                base_color: Color::GOLD,
                ..default()
            }),
        );

        return BlockMaterialStore { materials };
    }
}

impl BlockMaterialStore {
    pub fn get_material(&self, block_type: BlockType) -> Option<&Handle<StandardMaterial>> {
        self.materials.get(&block_type)
    }

    pub fn get_color(
        &self,
        block_type: BlockType,
        material_assets: &Res<Assets<StandardMaterial>>,
    ) -> Color {
        let material_handle = self.materials.get(&block_type);
        return material_assets
            .get(material_handle.unwrap())
            .unwrap()
            .base_color;
    }
}
