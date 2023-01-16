use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
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
            counts: HashMap::from([
                (Tile::BLUE, 0),
                (Tile::YELLOW, 0),
                (Tile::RED, 0),
                (Tile::BLACK, 0),
                (Tile::WHITE, 0),
            ])
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

pub fn tile_index(tile: Tile) -> usize {
    match tile {
        Tile::BLUE => 0,
        Tile::YELLOW => 1,
        Tile::RED => 2,
        Tile::BLACK => 3,
        Tile::WHITE => 4,
    }
}
