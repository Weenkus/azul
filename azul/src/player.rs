use std::cmp;

struct Player {
    floor_position: i32,
    rows: Vec<PatternRow>,
    wall: Wall,
    score: i32,
}

const MAX_NUM_ROWS: i32 = 5;

struct PatternRow {
    capacity: i32,
    size: i32,
    tile: Tile,
}

enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE,
}

// fn addRowTiles(player: Player, tile: Tile, num_tiles: u8, target_row: u8) {
//     w
// }

///

struct Wall {
    walls: Vec<Vec<bool>>
}

const WALL_PATTERN: [[Tile; 5]; 5] = [
    [Tile::BLUE, Tile::YELLOW, Tile::RED, Tile::BLACK, Tile::WHITE],
    [Tile::WHITE, Tile::BLUE, Tile::YELLOW, Tile::RED, Tile::BLACK],
    [Tile::BLACK, Tile::WHITE, Tile::BLUE, Tile::YELLOW, Tile::RED],
    [Tile::RED, Tile::BLACK, Tile::WHITE, Tile::BLUE, Tile::YELLOW],
    [Tile::YELLOW, Tile::RED, Tile::BLACK, Tile::WHITE, Tile::BLUE],
];

///

const FLOOR_MAX_PENTALY: i32 = -14;
const FLOOR_SCORES: [i32; 8] = [0, -1, -2, -4, -6, -8, -11, -14];

fn get_player_floor_score(player: Player) {
    cmp::min(FLOOR_SCORES[player.floor_position as usize], FLOOR_MAX_PENTALY);
}

fn add_player_floor_tiles(player: &mut Player, num_tiles: i32) {
    player.floor_position += num_tiles;
}

// let v = vec![100, 32, 57]; for i in &v { println!("i") }  
