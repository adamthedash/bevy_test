use crate::grid::Grid;
use crate::render::DisplayMesh;
use std::default::Default;

pub enum Block {
    Floor,
    Wall,
}

impl Default for Block {
    fn default() -> Self {
        Self::Floor
    }
}

impl DisplayMesh for Block {
    fn mesh_key(&self) -> &str {
        match self {
            Block::Floor => "entity_floor",
            Block::Wall => "entity_wall",
        }
    }
}

pub type World = Grid<Block>;

impl World {
    pub fn arena(size: (usize, usize)) -> Self {
        let mut world = Self::new(size);
        (0..world.size.0).for_each(|i| {
            *world.get_mut((i, 0)) = Block::Wall;
            *world.get_mut((i, world.size.1 - 1)) = Block::Wall;
        });
        (0..world.size.1).for_each(|j| {
            *world.get_mut((0, j)) = Block::Wall;
            *world.get_mut((world.size.1 - 1, j)) = Block::Wall;
        });

        world
    }
}
