use crate::game::tiles::*;
use std::cmp;


#[derive(Debug)]
pub struct  Player {
    pub floor_position: usize,
    pub rows: Vec<PatternRow>,
    pub wall: Vec<Vec<bool>>,
    pub score: i32,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            floor_position: 0,
            rows: (0..5).map(|i| PatternRow { capacity: i, size: 0, tile: None}).collect(),
            wall: vec![vec![false; 5]; 5],
            score: 0,
        }
    }
}

impl Player {
    pub fn any_row_complete(&self) -> bool {
        self.wall.iter()
            .any(|row| row.iter().all(|e| *e))
    }
}

fn is_valid_row_placement(player: &Player, row: &PatternRow, row_index: usize, tile: Tile) -> bool {
    if row.size >= row.capacity {
        return false;
    }

    match row.tile {
        // Check that a non-empty row can accept the tile
        Some(current_tile) => {
            if current_tile == tile {
                return true;
            }
        },
        // Check that an empty row can accept the tile
        // (has the tile empty on the wall)
        None => {
            let j = wall_row_tile_index(tile, row_index);
            if !player.wall[row_index][j] {
                return true;
            }
        },
    }

    return false;
}


pub fn execute_player_turn(player: &mut Player, tile: Tile, num_tiles: i32, target_row: usize) {
    assert!(num_tiles > 0, "num_tiles should be positive {}", num_tiles);

    let mut valid_action_exists = false;
    for (i, row) in player.rows.iter().enumerate() {
        valid_action_exists = is_valid_row_placement(player, row, i, tile);
    }

    // TODO return overflow tiles, remove if
    if !valid_action_exists {
        // Overflow - sadge
        player.floor_position += num_tiles as usize;
    } else {
        // Execute the placement
        add_row_tiles(player, tile, num_tiles, target_row);
        resolve_turn_patterns(player);
    }
}

pub const MAX_NUM_ROWS: i32 = 5;

// TODO(ivan) use hand bang instead
#[derive(Clone, Copy, Debug)]
pub struct PatternRow {
    pub capacity: i32,
    pub size: i32,
    pub tile: Option<Tile>,
}

fn reset_row(row: &mut PatternRow) {
    row.size = 0;
    row.tile = None;
}

fn add_row_tiles(player: &mut Player, tile: Tile, num_tiles: i32, target_row: usize) {
    // NOTE this assume this is already a valid action

    // Update the row
    let mut row = player.rows[target_row];
    let temp_size = row.size + num_tiles;

    // Check for row overflow
    let overflow = row.capacity - temp_size;
    if overflow > 0 {
        player.floor_position += overflow as usize;
    }

    // Rest the overflow
    row.size = cmp::max(temp_size, row.capacity);
}

fn resolve_turn_patterns(player: &mut Player) {
    for (i, row) in player.rows.iter_mut().enumerate() {
        if row.size == row.capacity {
            match row.tile {
                // Move to wall
                Some(tile) => {
                    let j = wall_row_tile_index(tile, i);
                    player.wall[i][j] = true;
                    reset_row(row);
                },
                None => assert!(true, "Full capacity rows should always have a tile!"),
            }
        }
    }
}

///

const WALL_PATTERN: [[Tile; 5]; 5] = [
    [Tile::BLUE, Tile::YELLOW, Tile::RED, Tile::BLACK, Tile::WHITE],
    [Tile::WHITE, Tile::BLUE, Tile::YELLOW, Tile::RED, Tile::BLACK],
    [Tile::BLACK, Tile::WHITE, Tile::BLUE, Tile::YELLOW, Tile::RED],
    [Tile::RED, Tile::BLACK, Tile::WHITE, Tile::BLUE, Tile::YELLOW],
    [Tile::YELLOW, Tile::RED, Tile::BLACK, Tile::WHITE, Tile::BLUE],
];

const FIRST_ROW: [Tile; 5] = [Tile::BLUE, Tile::YELLOW, Tile::RED, 
    Tile::BLACK, Tile::WHITE];

const NUM_COLORS: i32 = 5;

fn wall_row_tile_index(tile: Tile, num_row: usize) -> usize {
    let tile_index = tile_index(tile) + num_row;
    tile_index % (NUM_COLORS as usize)
}

///

const FLOOR_MAX_PENTALY: i32 = -14;
const FLOOR_SCORES: [i32; 8] = [0, -1, -2, -4, -6, -8, -11, -14];

fn get_player_floor_score(floor_position: usize) -> i32 {
    cmp::min(FLOOR_SCORES[floor_position as usize], FLOOR_MAX_PENTALY)
}

fn add_player_floor_tiles(player: &mut Player, num_tiles: i32) {
    player.floor_position += num_tiles as usize;
}

pub fn clear_rows(player: &mut Player) {
    // Count the rows to overflow
    let overflow: usize = player.rows.iter()
        .map(|f| f.size as usize)
        .sum();

    player.floor_position += overflow;
    player.score += get_player_floor_score(player.floor_position) as i32;

    // Clean the rows
    player.rows.iter_mut()
        .for_each(reset_row);
}
