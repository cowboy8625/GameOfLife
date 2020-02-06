use std::ops::{Index, IndexMut};
use crate::alive;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<bool>,
}

impl Grid {
    pub fn from_fn(width: usize, height: usize, f: impl FnMut() -> bool) -> Self {
        Self {
            width,
            height,
            tiles: std::iter::repeat_with(f).take(width * height).collect()
        }
    }

    pub fn sub_grid(&self, x: usize, y: usize, width: usize, height: usize) -> Self {
        let sub_tiles = (0..height)
            .flat_map(|dy|
                (0..width)
                    .map(move |dx| self[y + dy][x + dx])
            )
            .collect();

        Self {
            width,
            height,
            tiles: sub_tiles,
        }
    }

    pub fn next_gen(&mut self) {
        let r: &Grid = &self;
        self.tiles = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| alive(x as i32, y as i32, r)))
            .collect();
    }
}

impl Index<usize> for Grid {
    type Output = [bool];

    fn index(&self, idx: usize) -> &Self::Output {
        let idx_start = idx * self.width;
        let idx_end = idx_start + self.width;

        self.tiles.index(idx_start..idx_end)
    }
}

impl IndexMut<usize> for Grid {

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let idx_start = idx * self.width;
        let idx_end = idx_start + self.width;

        self.tiles.index_mut(idx_start..idx_end)
    }
}
