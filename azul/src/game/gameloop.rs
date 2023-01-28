use crate::game::tiles::*;
use crate::game::factory::*;
use crate::game::player::*;

use rand::Rng;


pub struct GameState {
    pub player_count: usize,
    pub tile_bag: TileSet,
    pub factory_floor: FactoryFloor,

    pub current_round: usize,
    pub current_player_turn: usize,

    pub players: Vec<Player>,
}
impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_count: 0,
            current_round: 0,
            current_player_turn: 0,
            tile_bag: TileSet::create_starting_bag(),
            factory_floor: FactoryFloor::default(),
            players: vec![Player::default(), Player::default()]
        }
    }
}

pub fn set_up_game(game_state: &mut GameState) {
    println!("Setting up game");

    game_state.player_count = 2;
    game_state.factory_floor.displays = (0..5).map(|i| TileSet::default()).collect();
}

pub fn end_game(game_state: &GameState) {
    println!("Ending game");
}

pub fn set_up_round(game_state: &mut GameState) {
    println!("Setting up round");

    game_state.tile_bag.take_random_n(3);
}

pub fn end_round(game_state: &mut GameState) {
    println!("Ending round");

    println!("Clearning rows, filling floor, scoring");
    game_state.players.iter_mut()
        .for_each(clear_rows)
}

pub fn is_game_end(game_state: &GameState) -> bool {
    // game_state.players.iter()
    //    .any(|player| player.any_row_complete())

    game_state.current_round >= 2
}

pub fn is_round_end(game_state: &GameState) -> bool {
    // game_state.factory_floor.is_empty()

    game_state.current_player_turn >= 1
}

pub fn update_ui(game_state: &GameState) {
    println!("Updating UI");
}

pub fn take_turn(game_state: &mut GameState) {
    println!("Player {} turn", game_state.current_player_turn);

    let mut current_player = &mut game_state.players[game_state.current_player_turn];

    // TODO(ivan) pick tiles
    let random_tile = Tile::BLACK;
    let random_num_tiles = 2;

    // Simulate picking a random row
    let mut rng = rand::thread_rng();
    let random_row: usize= rng.gen_range(0..MAX_NUM_ROWS) as usize;

    execute_player_turn(current_player, random_tile, random_num_tiles, random_row);
}