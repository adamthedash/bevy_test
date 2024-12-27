use crate::entities::DisplayMesh;
mod assets;
mod entities;
mod hello;
mod world;

use crate::assets::load_entity_meshes;
use assets::MaterialStore;
use assets::MeshStore;
use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use entities::Entity;
use world::Grid;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let world = default_grid((21, 21));
        app.insert_resource(world)
            .insert_resource(MeshStore::new())
            .insert_resource(MaterialStore::new())
            .add_systems(
                Startup,
                (print_world, (load_entity_meshes, draw_world).chain()),
            );
    }
}

/// Set up a default world, an empty arena with the player in the middle
fn default_grid(size: (usize, usize)) -> Grid<crate::entities::Entity> {
    let mut world = Grid::<crate::entities::Entity>::new(size);
    (0..world.size.0).for_each(|i| {
        *world.get_mut((i, 0)) = Entity::Wall;
        *world.get_mut((i, world.size.1 - 1)) = Entity::Wall;
    });
    (0..world.size.1).for_each(|j| {
        *world.get_mut((0, j)) = Entity::Wall;
        *world.get_mut((world.size.1 - 1, j)) = Entity::Wall;
    });
    *world.get_mut((world.size.0 / 2, world.size.1 / 2)) = Entity::Player;
    *world.get_mut((4, 4)) = Entity::Enemy;

    world
}

fn draw_world(
    mut commands: Commands,
    asset_store: ResMut<MeshStore>,
    material_store: ResMut<MaterialStore>,
    world: Res<Grid<crate::entities::Entity>>,
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

fn print_world(world: Res<Grid<crate::entities::Entity>>) {
    println!("{}", world.into_inner());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Wireframe2dPlugin)
        .add_plugins(GamePlugin)
        .run();
}
