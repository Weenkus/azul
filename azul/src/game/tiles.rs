use std::collections::HashMap;
use rand::prelude::*;
use std::cmp;


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE
}

#[derive(Debug, Clone)]
pub struct TileSet {
    pub tiles: HashMap<Tile, i32>
}
impl Default for TileSet {
    fn default() -> Self {
        Self {
            tiles: HashMap::from([
                (Tile::BLUE, 0),
                (Tile::YELLOW, 0),
                (Tile::RED, 0),
                (Tile::BLACK, 0),
                (Tile::WHITE, 0),
            ])
        }
    }
}
impl TileSet {
    pub fn new(tile: Tile, count: i32) -> Self {
        let mut tile_set: TileSet = Default::default();
        tile_set.insert_tiles(tile, count);
        tile_set
    }
    pub fn create_starting_bag() -> Self {
        Self {
            tiles: HashMap::from([
                (Tile::BLUE, 20),
                (Tile::YELLOW, 20),
                (Tile::RED, 20),
                (Tile::BLACK, 20),
                (Tile::WHITE, 20),
            ])
        }
    }

    pub fn available_colors(&self) -> Vec<Tile> {
        self.tiles.iter()
            .filter(|(k, v)| **v > 0)
            .map(|(k, v)| k.clone())
            .collect()
    }

    pub fn take_all_color(mut self, tile: Tile) -> TileSet {

        let color_set = TileSet {
            tiles: HashMap::from([(tile, self.tiles[&tile])])
        };
        self.tiles.insert(tile, 0);
        color_set
    }

    pub fn total_tiles_count(&self) -> i32 {
        (&self.tiles).iter().map(|(k, v)| v).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_tiles_count() == 0
    }

    pub fn take_n_tiles(&mut self, tile: Tile, count: i32) -> TileSet {
        let old = self.tiles.get(&tile).unwrap().clone();
        print!("{} {} {}", old, count, old - count);
        self.tiles.insert(tile, old - count);
        TileSet::new(tile, count)
    }

    pub fn insert_tiles(&mut self, tile: Tile, count:i32) {
        self.tiles.insert(tile, count);
    }

    pub fn add(&mut self, other: TileSet) {
        for (tile, o_n) in other.tiles {
            let s_n = self.tiles.get(&tile).unwrap();
            self.tiles.insert(tile, o_n + s_n);
        }
    }

    pub fn take_random_n(&mut self, n: i32) -> TileSet {
        let mut selection_bag = TileSet::default();
        let actual_n = cmp::min(n, self.total_tiles_count());

        for _i in 0..actual_n {
            let random_number: f64 = rand::thread_rng().gen();

            let mut pick_index = ((self.total_tiles_count() as f64) * random_number) as i32;

            let mut pick_tile = Tile::BLACK;
            for (tile, count) in &self.tiles {
                if *count < pick_index {
                    pick_index -= *count
                } else if *count > 0 {
                    pick_tile = tile.clone();
                    break;
                }
            }

            self.tiles.insert(pick_tile, self.tiles[&pick_tile] - 1);
            selection_bag.tiles.insert(pick_tile, selection_bag.tiles[&pick_tile] + 1);
        }

        selection_bag
    }

}

pub fn tile_index(tile: Tile) -> usize {
    match tile {
        Tile::BLUE => 0,
        Tile::YELLOW => 1,
        Tile::RED => 2,
        Tile::BLACK => 3,
        Tile::WHITE => 4,
    }
}
