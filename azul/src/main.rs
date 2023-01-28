mod game;
use crate::game::gameloop::*;

fn main() {
    println!("Welcome to most generic Azul implementation");

    let mut game_state = GameState::default();

    set_up_game(&mut game_state);

    while !is_game_end(&game_state) {
        set_up_round(&mut game_state);

        while !is_round_end(&game_state) {
            take_turn(&mut game_state);
            update_ui(&game_state);
            
            game_state.current_player_turn = (game_state.current_player_turn + 1) % game_state.player_count;
        }
        
        end_round(&mut game_state);
        game_state.current_round += 1;
    }
    end_game(&game_state);
}
