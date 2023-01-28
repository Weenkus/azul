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

#[derive(Debug)]
pub struct TileSet {
    pub counts: HashMap<Tile, i32>
}
impl Default for TileSet {
    fn default() -> Self {
        Self {
            counts: HashMap::from([
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
    pub fn create_starting_bag() -> Self {
        Self {
            counts: HashMap::from([
                (Tile::BLUE, 20),
                (Tile::YELLOW, 20),
                (Tile::RED, 20),
                (Tile::BLACK, 20),
                (Tile::WHITE, 20),
            ])
        }
    }

    pub fn take_all_color(mut self, tile: Tile) -> TileSet {

        let color_set = TileSet {
            counts: HashMap::from([(tile, self.counts[&tile])])
        };
        self.counts.insert(tile, 0);
        color_set
    }

    pub fn total_tiles_count(&self) -> i32 {
        (&self.counts).iter().map(|(k, v)| v).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.total_tiles_count() == 0
    }

    pub fn take_n_tiles(mut self, tile: Tile, count: i32) -> TileSet {
        self.counts.insert(tile, count);
        let old = self.counts.get_mut(&tile);
        match old {
            Some(o2) => {
                *o2 -= count;
                Self {
                    counts: HashMap::from([
                        (tile, *o2),
                    ])
                }
            } 
            None => {
                TileSet::default()
            }
        }
    }

    pub fn insert_tiles(mut self, tile: Tile, count:i32) {
        self.counts.insert(tile, count);
    }

    pub fn append(mut self, bag: TileSet) {

    }

    pub fn take_random_n(&mut self, n: i32) -> TileSet {
        let mut selection_bag = TileSet::default();
        let actual_n = cmp::min(n, self.total_tiles_count());

        for i in 0..actual_n {
            let random_number: f64 = rand::thread_rng().gen();

            let mut pick_index = ((self.total_tiles_count() as f64) * random_number) as i32;

            let mut pick_tile = Tile::BLACK;
            for (tile, count) in &self.counts {
                if *count < pick_index {
                    pick_index -= *count
                } else if *count > 0 {
                    pick_tile = tile.clone();
                    break;
                }
            }

            self.counts.insert(pick_tile, self.counts[&pick_tile] - 1);
            selection_bag.counts.insert(pick_tile, selection_bag.counts[&pick_tile] + 1);
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
