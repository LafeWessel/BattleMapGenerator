use map_generator::battle_map::Map;

fn main() {

    for i in 4..=5{
        for j in 2..=5{
            let m = Map::create_map(i,j);
            m.print_board_tiles();
        }
    }

    Map::create_default_map().print_board_tiles();

    let m = Map::create_map(4, 6, vec![]);
    m.print_board();


    for i in 4..=5{
        for j in 2..=5{
            let m = Map::create_map(i,j);
            m.print_board_owners();
        }
    }

    Map::create_default_map().print_board_owners();

}
