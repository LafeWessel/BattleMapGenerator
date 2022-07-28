use map_generator::map::Map;

fn main() {

    for i in 2..=5{
        for j in 2..=5{
            let m = Map::create_map(i,j);
            m.print_board();
        }
    }

    // let m = Map::create_map(4, 6, vec![]);
    // m.print_board();

}
