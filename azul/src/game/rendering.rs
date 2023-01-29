use colored::Colorize;
use super::{player::Player};

fn print_player_board(player: &Player) {
    let c = player.rows.len();
    for (i, row) in player.rows.iter().enumerate() {
        let mut rs = " ".repeat(c - i - 1);
        rs.push_str(&".".repeat(i+1));

        rs.push_str(" > ");

        for flag in &player.wall[i] {
            if *flag {
                rs.push('X');
            } else {
                rs.push('.');
            }
        }

        println!("{}", rs)
    }
}

pub fn print_player_boards(players: &Vec<Player>) {
    for (i, player) in players.iter().enumerate() {
        println!("PLAYER #{}", i);
        print_player_board(player);
        println!();
    }
}