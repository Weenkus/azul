use colored::{Colorize, Color};
use std::collections::HashMap;
use crate::game::player::WALL_PATTERN;

use super::{player::Player, tiles::Tile};

fn print_player_board(player: &Player) {
    let c = player.rows.len();
    for (i, row) in player.rows.iter().enumerate() {
        let tile_to_color: HashMap<Tile, Color> = HashMap::from([
            (Tile::BLUE,    Color::BrightBlue),
            (Tile::YELLOW,  Color::BrightYellow),
            (Tile::RED,     Color::BrightRed),
            (Tile::BLACK,   Color::BrightBlack),
            (Tile::WHITE,   Color::BrightWhite),
        ]);

        // Empty padding
        print!("{}",  "  ".repeat(c - i - 1));

        // Empty tiles
        print!("{}", "[]".repeat(i + 1 - (row.size as usize)).color(Color::TrueColor { r: 219, g: 147, b: 112 }));

        // Filled tiles
        if let Some(tile) = row.tile {
            print!("{}", "  ".repeat(row.size as usize).on_color(tile_to_color[&tile]));            
        }

        // Separator
        print!(" > ");

        // Wall tiles
        for (j, flag) in player.wall[i].iter().enumerate() {
            if *flag {
                print!("{}", " ".on_color(tile_to_color[&WALL_PATTERN[i][j]]));
            } else {
                print!("{}", "[]".color(tile_to_color[&WALL_PATTERN[i][j]]));
            }
        }

        println!();
    }
}

pub fn print_player_boards(players: &Vec<Player>) {
    for (i, player) in players.iter().enumerate() {
        println!("PLAYER #{}", i);
        print_player_board(player);
        println!();
    }
}