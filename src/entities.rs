use std::default::Default;

pub enum Entity {
    Floor,
    Wall,
    Player,
    Enemy,
}

impl Default for Entity {
    fn default() -> Self {
        Self::Floor
    }
}

impl DisplayGlyph for Entity {
    fn glyph(&self) -> char {
        match self {
            Entity::Floor => 'F',
            Entity::Wall => 'W',
            Entity::Player => 'P',
            Entity::Enemy => 'E',
        }
    }
}
impl DisplayMesh for Entity {
    fn mesh_key(&self) -> &str {
        match self {
            Entity::Floor => "entity_floor",
            Entity::Wall => "entity_wall",
            Entity::Player => "entity_player",
            Entity::Enemy => "entity_enemy",
        }
    }
}
pub trait DisplayGlyph {
    /// Returns a character used to represent the entity in strings
    fn glyph(&self) -> char;
}

pub trait DisplayMesh {
    /// Returns the identifier key for this object which corresponds to a loaded mesh. Should be
    /// globally unique amongst object types.
    fn mesh_key(&self) -> &str;
}
