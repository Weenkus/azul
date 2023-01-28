use crate::game::tiles::*;

use std::iter;

use super::gameloop::GameState;

pub const NUM_DISPLAYS: i32 = 6;
pub const NUM_TILES_PER_DISPLAY: i32 = 4;


#[derive(Debug)]
pub struct FactoryFloor {
    pub displays: Vec<TileSet>,
    pub center: TileSet,
}
impl Default for FactoryFloor {
    fn default() -> Self {
        let mut displays = Vec::with_capacity(NUM_DISPLAYS as usize);
        (0..NUM_DISPLAYS)
            .for_each(|f| displays.push(TileSet::default()));
        Self {
            displays: displays,
            center: TileSet::default()
        }
    }
}

impl FactoryFloor {
    pub fn refill(&mut self, tile_bag: &mut TileSet) {
        for i in 0..self.displays.len() {
            let display_tile_set = tile_bag.take_random_n(NUM_TILES_PER_DISPLAY);
            self.displays[i] = display_tile_set;
        }
    }

    pub fn is_empty(&self) -> bool {
        iter::once(&self.center)
            .chain(&self.displays)
            .all(|tile_set| tile_set.is_empty())
    }
}
