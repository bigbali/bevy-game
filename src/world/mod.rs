use bevy::prelude::{
    App, AssetServer, Assets, Commands, Image, Mesh, Plugin, Res, ResMut, StandardMaterial,
};
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

// use crate::chunk::CHUNK_SIZE;

use crate::ui::debug::initialize_heightmap_overlay;

use self::{block::*, chunk::CHUNK_SIZE};

pub mod block;
pub mod chunk;

const INITIAL_WORLD_SIZE_FOR_TESTING: usize = 16;
const HEIGHTMAP_SIZE: usize = CHUNK_SIZE * INITIAL_WORLD_SIZE_FOR_TESTING;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BlockPlugin)
            .add_startup_system(generate_chunks);
    }
}

pub fn generate_chunks(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    material_store: ResMut<BlockMaterialStore>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    materials: Res<Assets<StandardMaterial>>,
) {
    // with this we generate the terrain
    let mut height_data = Vec::with_capacity(HEIGHTMAP_SIZE * HEIGHTMAP_SIZE);
    // and with this we render the heightmap to the UI
    let mut rgba_data = Vec::with_capacity(HEIGHTMAP_SIZE * HEIGHTMAP_SIZE * 4);

    let noise = get_perlin_noise();

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
        for z in 0..INITIAL_WORLD_SIZE_FOR_TESTING {}
    }

    initialize_heightmap_overlay(
        commands,
        images,
        HEIGHTMAP_SIZE as u32,
        HEIGHTMAP_SIZE as u32,
        rgba_data,
    )
}

// pub fn generate_simple_mesh(samples: &[Block]) -> Mesh {
//     let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

//     let mut buffer = UnitQuadBuffer::new();
//     visible_block_faces(
//         samples,
//         &ChunkShape {},
//         [0; 3],
//         [33; 3],
//         &faces,
//         &mut buffer,
//     );

//     let num_indices = buffer.num_quads() * 6;
//     let num_vertices = buffer.num_quads() * 4;
//     let mut indices = Vec::with_capacity(num_indices);
//     let mut positions = Vec::with_capacity(num_vertices);
//     let mut normals = Vec::with_capacity(num_vertices);
//     let mut uvs = Vec::with_capacity(num_vertices);
//     let mut colors = Vec::with_capacity(num_vertices);
//     for (group, face) in buffer.groups.into_iter().zip(faces.into_iter()) {
//         for quad in group.into_iter() {
//             let quad_positions = face.quad_mesh_positions(&quad.into(), 1.0);
//             indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
//             positions.extend_from_slice(&quad_positions);
//             normals.extend_from_slice(&face.quad_mesh_normals());

//             let normal = Vec3::from_array(face.quad_mesh_normals()[0]);
//             let voxel_type = face_to_voxel_type(samples, face, quad_positions);

//             let default_color = [[1.0, 1.0, 1.0, 1.0]; 4];
//             let color = match voxel_type {
//                 BlockType::Grass => {
//                     if normal.y == 1.0 {
//                         [[0.1, 0.8, 0.1, 1.0]; 4]
//                     } else {
//                         default_color
//                     }
//                 }
//                 BlockType::OakLeaves => [[0.1, 0.8, 0.1, 1.0]; 4],
//                 _ => default_color,
//             };
//             colors.extend_from_slice(&color);

//             let frame_name = voxel_texture_name(normal, voxel_type);

//             uvs.extend_from_slice(&atlas_uv(
//                 atlas,
//                 &atlas.frames.get(frame_name).unwrap().frame,
//             ));
//         }
//     }
//     let generated = positions.len();
//     let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
//     render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//     render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
//     render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//     render_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
//     render_mesh.set_indices(Some(Indices::U32(indices.clone())));
//     (render_mesh, generated)
// }

pub fn get_perlin_noise() -> NoiseMap {
    let fbm = Fbm::<Perlin>::new(999);

    let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(HEIGHTMAP_SIZE, HEIGHTMAP_SIZE)
        .build();

    return noise;
}
