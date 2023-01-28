
mod game;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn pop_color_test() {
        let testSet = TileSet::create_starting_bag();
        let out = testSet.pop_color(Tile::BLACK);
        out.print();
    }
}