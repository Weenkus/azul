mod gameloop;
use crate::gameloop::*;

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
