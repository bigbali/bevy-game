// use bevy::prelude::*;

// pub fn get_target_with_normals(
//     camera: &Camera,
//     camera_transform: &GlobalTransform,
//     target_point: Vec2,
//         rapier_context: Res<RapierContext>,

// ) {
//     let ray = camera
//         .viewport_to_world(camera_transform, target_point)
//         .unwrap();

//     if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
//         ray.origin,
//         ray.direction,
//         f32::MAX,
//         true,
//         QueryFilter::new(),
//     ) {
//         let hit_point = intersection.point;
//         let hit_normal = intersection.normal;
//         println!("Entity {:?} hit at point {:?}", entity, intersection);

//         let color = Color::BLUE;
//         commands.entity(entity).insert(ColliderDebugColor(color));

//         // MaterialMeshBundle
//         let mesh = Mesh::from(shape::Cube { size: 1.0 });
//         let block = meshes.add(mesh.clone());

//         commands.spawn(Block {
//             render: PbrBundle {
//                 mesh: block,
//                 material: materials.add(Color::rgb(0.9, 1.0, 1.0).into()),
//                 transform: Transform::from_xyz(
//                     hit_point.x + hit_normal.x,
//                     hit_point.y + hit_normal.y,
//                     hit_point.z + hit_normal.z,
//                 ),
//                 ..default()
//             },
//             collider: Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap(),
//         });
//     }
// }
