mod assets;
mod grid;
mod render;
mod world;

use crate::assets::load_entity_meshes;
use crate::world::World as MyWorld;
use assets::MaterialStore;
use assets::MeshStore;
use bevy::prelude::*;
use bevy::sprite::Wireframe2dPlugin;
use render::draw_world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyWorld::arena((21, 21)))
            .insert_resource(MeshStore::new())
            .insert_resource(MaterialStore::new())
            .add_systems(Startup, (load_entity_meshes, draw_world).chain());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Wireframe2dPlugin)
        .add_plugins(GamePlugin)
        .run();
}
