use crate::game::tiles::*;

pub struct FactoryFloor {
    pub displays: Vec<TileBag>,
    pub center: TileBag,
}
impl Default for FactoryFloor {
    fn default() -> Self {
        Self {
            displays: Vec::new(),
            center: TileBag::default()
        }
    }
}