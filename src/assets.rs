use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

/// Used to store meshes which are re-used by instances of objects.
#[derive(Resource)]
pub struct MeshStore(pub HashMap<String, Handle<Mesh>>);

impl MeshStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for MeshStore {
    type Target = HashMap<String, Handle<Mesh>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MeshStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Used to store colours which are re-used by instances of objects.
#[derive(Resource)]
pub struct MaterialStore(pub HashMap<String, Handle<ColorMaterial>>);

impl MaterialStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for MaterialStore {
    type Target = HashMap<String, Handle<ColorMaterial>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Load all the assets used to draw the world
pub fn load_entity_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_store: ResMut<MeshStore>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut material_store: ResMut<MaterialStore>,
) {
    // Entities
    println!("Loading meshes");
    let square = Rectangle::new(1., 1.);
    mesh_store.insert("entity_wall".to_string(), meshes.add(square));
    mesh_store.insert("entity_floor".to_string(), meshes.add(square));
    mesh_store.insert("entity_player".to_string(), meshes.add(square));
    mesh_store.insert("entity_enemy".to_string(), meshes.add(square));

    println!("Loading colors");
    material_store.insert(
        "entity_wall".to_string(),
        materials.add(Color::srgb(0.25, 0.25, 0.25)),
    );
    material_store.insert(
        "entity_floor".to_string(),
        materials.add(Color::srgb(0.75, 0.75, 0.75)),
    );
    material_store.insert(
        "entity_player".to_string(),
        materials.add(Color::srgb(0., 0.75, 0.)),
    );
    material_store.insert(
        "entity_enemy".to_string(),
        materials.add(Color::srgb(0.75, 0., 0.)),
    );
}
