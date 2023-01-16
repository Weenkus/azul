use crate::game::tiles::*;
use crate::game::factory::*;


pub struct GameState {
    pub player_count: i32,
    pub tile_bag: TileBag,
    pub factory_floor: FactoryFloor,

    pub current_round: i32,
    pub current_player_turn: i32,
}
impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_count: 0,
            current_round: 0,
            current_player_turn: 0,
            tile_bag: TileBag::create_starting_bag(),
            factory_floor: FactoryFloor::default(),
        }
    }
}

pub fn setup_game(game_state: &mut GameState) {
    println!("Setting up game");

    game_state.player_count = 2;
    game_state.factory_floor.displays = (0..5).map(|i| TileBag::default()).collect();
}

pub fn end_game(game_state: &GameState) {
    println!("Ending game");
}

pub fn setup_round(game_state: &GameState) {
    println!("Setting up round");
}

pub fn end_round(game_state: &GameState) {
    println!("Ending round");
}

pub fn is_game_end(game_state: &GameState) -> bool {
    game_state.current_round >= 2
}

pub fn is_round_end(game_state: &GameState) -> bool {
    game_state.current_player_turn >= 1
}

pub fn update_ui(game_state: &GameState) {
    println!("Updating UI");
}

pub fn take_turn(game_state: &GameState) {
    println!("Player {} turn", game_state.current_player_turn);
}