use std::collections::HashMap;
use rand::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    BLUE,
    YELLOW,
    RED,
    BLACK,
    WHITE
}

pub struct TileBag {
    pub counts: HashMap<Tile, i32>
}
impl Default for TileBag {
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
impl TileBag {
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

    pub fn total_tiles_count(&self) -> i32 {
        (&self.counts).iter().map(|(k, v)| v).sum()
    }

    pub fn append(mut self, bag: TileBag) {

    }

    pub fn take_random_n(mut self, n: i32) -> TileBag {
        let mut selection_bag = TileBag::default();
        
        for i in 0..n {
            println!("   Take i={}", i);

            let random_number: f64 = rand::thread_rng().gen();
            let mut pick_index =  (((&self).total_tiles_count() as f64) * random_number) as i32;
             
            println!("   Take pick_index={}", pick_index);

            // TODO: Continue the algorithme to pick here
            // for (tile, count) in &self.counts {
            //     if count <= &pick_index {
            //         pick_index -= count;
            //     } else {
                    
            //     }
            // }
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
