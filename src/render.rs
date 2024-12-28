use bevy::{
    math::Vec3,
    prelude::{Camera2d, Commands, Mesh2d, Res, ResMut, Transform},
    sprite::MeshMaterial2d,
};

use crate::{
    assets::{MaterialStore, MeshStore},
    world::World,
};

pub trait DisplayGlyph {
    /// Returns a character used to represent the entity in strings
    fn glyph(&self) -> char;
}

pub trait DisplayMesh {
    /// Returns the identifier key for this object which corresponds to a loaded mesh. Should be
    /// globally unique amongst object types.
    fn mesh_key(&self) -> &str;
}

pub fn draw_world(
    mut commands: Commands,
    asset_store: ResMut<MeshStore>,
    material_store: ResMut<MaterialStore>,
    world: Res<World>,
) {
    commands.spawn(Camera2d);

    println!("Drawing world");
    let SCREEN_SIZE = (900_f32, 1600_f32);

    // Loop over each pixel
    (0..world.size.0)
        .flat_map(|i| {
            (0..world.size.1)
                .map(|j| (i, j, world.get((i, j))))
                .collect::<Vec<_>>()
        })
        .for_each(|(i, j, cell)| {
            // Load the graphic materials
            let mesh = asset_store.get(cell.mesh_key()).expect("Mesh not loaded!");
            let color = material_store
                .get(cell.mesh_key())
                .expect("Color not loaded!");

            // Calculate the entity transform
            let transform = Transform::from_xyz(
                // In a square grid, fit to the smallest screen axis
                ((j as f32 / world.size.1 as f32) - 0.5) * SCREEN_SIZE.1.min(SCREEN_SIZE.0),
                ((i as f32 / world.size.0 as f32) - 0.5) * SCREEN_SIZE.1.min(SCREEN_SIZE.0),
                0.0,
            )
            .with_scale(Vec3::new(
                // Squares
                SCREEN_SIZE.1.min(SCREEN_SIZE.0) / world.size.1 as f32,
                SCREEN_SIZE.1.min(SCREEN_SIZE.0) / world.size.0 as f32,
                1.0,
            ));

            // Draw it
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
                transform,
            ));
        });
}
