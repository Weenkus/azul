#[derive(PartialEq, Eq)]
pub enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE,
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
