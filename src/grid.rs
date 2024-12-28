use std::fmt::Display;

use bevy::prelude::Resource;

use crate::render::DisplayGlyph;

#[derive(Resource)]
pub struct Grid<T> {
    pub size: (usize, usize), // (height, width)
    data: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            data: (0..size.0 * size.1).map(|_| T::default()).collect(),
        }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, pos: (usize, usize)) -> &T {
        assert!(
            pos.0 < self.size.0 && pos.1 < self.size.1,
            "Position is out of bounds: pos {:?}, world {:?}",
            pos,
            self.size
        );

        let index = pos.0 * self.size.0 + pos.1;

        &self.data[index]
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> &mut T {
        assert!(
            pos.0 < self.size.0 && pos.1 < self.size.1,
            "Position is out of bounds: pos {:?}, world {:?}",
            pos,
            self.size
        );

        let index = pos.0 * self.size.0 + pos.1;

        &mut self.data[index]
    }
}

impl<T: DisplayGlyph> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.chunks_exact(self.size.0).try_for_each(|row| {
            let row = row.iter().map(DisplayGlyph::glyph).collect::<String>();
            writeln!(f, "{}", row)
        })
    }
}
