pub struct GameState {
    pub player_count: i32,

    pub current_round: i32,
    pub current_player_turn: i32,
}
impl Default for GameState {
    fn default() -> Self {
        Self { 
            player_count: 0,
            current_round: 0,
            current_player_turn: 0,
        }
    }
}

pub fn setup_game(game_state: &GameState) {
    println!("Setting up game");
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

pub fn is_game_finished(game_state: &GameState) -> bool {
    game_state.current_round < 2
}

pub fn update_ui(game_state: &GameState) {
    println!("Updating UI");
}

pub fn take_turn(game_state: &GameState) {
    println!("Player {} turn", game_state.current_player_turn);
}