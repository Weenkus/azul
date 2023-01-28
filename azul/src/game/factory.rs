use crate::game::tiles::*;

use std::iter;

pub struct FactoryFloor {
    pub displays: Vec<TileSet>,
    pub center: TileSet,
}
impl Default for FactoryFloor {
    fn default() -> Self {
        Self {
            displays: Vec::new(),
            center: TileSet::default()
        }
    }
}

impl FactoryFloor {
    pub fn is_empty(&self) -> bool {
        iter::once(&self.center)
            .chain(&self.displays)
            .all(|tile_set| tile_set.is_empty())
    }
}
