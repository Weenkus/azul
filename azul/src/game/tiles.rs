use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE
}

pub struct TileBag {
    pub counts: HashMap<Tile, i32>
}
impl Default for TileBag {
    fn default() -> Self {
        Self {
            counts: HashMap::default()
        }
    }
}
impl TileBag {
    pub fn create_starting_bag() -> Self {
        Self {
            counts: HashMap::from([
                (Tile::BLUE, 20),
                (Tile::YELLOW, 20),
                (Tile::RED, 20),
                (Tile::BLACK, 20),
                (Tile::WHITE, 20),
            ])
        }
    }
}