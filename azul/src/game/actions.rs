use super::gameloop::GameState;
use super::tiles::Tile;

use std::iter;
use itertools::{iproduct, Itertools};


#[derive(Debug, Clone, Copy)]
pub struct TakeTileAction {
    /// Represents the take action for a player
    /// If the player takes from the center of the factory floor,
    /// the display_index will be None, otherwise if it takes from
    /// the display, the display_index will be set
    display_index: Option<i32>,
    tile: Tile,
}

#[derive(Debug, Clone, Copy)]
pub struct PutTileAction {
    row_index: i32    
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub take_action: TakeTileAction,
    pub put_action: PutTileAction,
}

pub fn available_actions(game_state: &GameState) -> Vec<Action> {
    iproduct!(
        available_take_actions(game_state).iter(),
        availalbe_put_actions(game_state).iter()
    )
        .map(|(take_action, put_action)| Action{ 
            take_action: take_action.clone(), 
            put_action: put_action.clone(),
        })
        .collect::<Vec<Action>>()
}

pub fn available_take_actions(game_state: &GameState) -> Vec<TakeTileAction> {
    iter::once(game_state.factory_floor.center.clone())
        .chain(game_state.factory_floor.displays.clone())
        .enumerate()
        .map(|(i, tile_set)| iproduct!(iter::once((i, tile_set.is_empty())), tile_set.available_colors()))
        .flatten()
        .map(|((i, is_empty), tile)| {
            TakeTileAction{ 
                display_index: if is_empty {None} else {Some(i as i32)},
                tile: tile,
            }
        })
        .collect::<Vec<TakeTileAction>>()
}

pub fn availalbe_put_actions(game_state: &GameState) -> Vec<PutTileAction> {
    game_state.players.get(game_state.current_player_turn)
        .iter()
        .flat_map(|player| player.rows.clone())
        .filter(|row| !row.is_empty())
        .map(|row| PutTileAction{ row_index: row.capacity-1 })
        .collect::<Vec<PutTileAction>>()
}