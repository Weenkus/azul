fn main() {
    println!("Welcome to most generic Azul implementation");

    let mut game_state = GameState::default();
    game_state.player_count = 2;

    setup_game(&game_state);

    while is_game_finished(&game_state) {
        setup_round(&game_state);

        while game_state.current_player_turn < game_state.player_count {
            take_turn(&game_state);
            update_ui(&game_state);
            game_state.current_player_turn += 1;
        }
        
        end_round(&game_state);
        game_state.current_round += 1;
    }
    end_game(&game_state);
}

struct GameState {
    player_count: i32,

    current_round: i32,
    current_player_turn: i32,
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

fn setup_game(game_state: &GameState) {
    println!("Setting up game");
}

fn end_game(game_state: &GameState) {
    println!("Ending game");
}

fn setup_round(game_state: &GameState) {
    println!("Setting up round");
}

fn end_round(game_state: &GameState) {
    println!("Ending round");
}

fn is_game_finished(game_state: &GameState) -> bool {
    game_state.current_round < 2
}

fn update_ui(game_state: &GameState) {
    println!("Updating UI");
}

fn take_turn(game_state: &GameState) {
    println!("Player {} turn", game_state.current_player_turn);

}