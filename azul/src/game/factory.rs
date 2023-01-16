use crate::game::tiles::*;

pub struct FactoryFloor {
    pub displays: Vec<TileBag>
}
impl Default for FactoryFloor {
    fn default() -> Self {
        Self {
            displays: Vec::new()
        }
    }
}