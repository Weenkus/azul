use crate::game::tiles::*;

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