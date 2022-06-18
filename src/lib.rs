
mod tiles{

    pub enum CampaignMapTileType{
        Forest,
        Hill,
        Mountain,
        Outpost,
        Plains,
        River,
        Road,
        Swamp,
        Town,
    }

    // TODO add any missing tile types that are in the base game
    #[derive(Clone)]
    pub enum BattleMapTileType{
        Forest,
        Hill,
        Mountain,
        Outpost,
        Plains,
        River,
        Road,
        Swamp,
        Town,
    }

    impl BattleMapTileType{
        pub fn to_console_string(&self) -> String{
            String::from(match &self{
                BattleMapTileType::Forest => "F",
                BattleMapTileType::Hill => "H",
                BattleMapTileType::Mountain => "M",
                BattleMapTileType::Outpost => "O",
                BattleMapTileType::Plains => "P",
                BattleMapTileType::River => "R",
                BattleMapTileType::Road => "V",
                BattleMapTileType::Swamp => "S",
                BattleMapTileType::Town => "T",
            })
        }

    }

    enum RoadTileType{
        Straight, // has 3 rotations
        Turn120, // has 6 rotations
        BranchY, // has 2 rotations
        Branch120, // has 6 rotations
    }

    enum RiverTileType{
        Straight, // has 3 rotations
        Turn120, // has 6 rotations
        BranchY, // has 2 rotations
        Branch120, // has 6 rotations
        Lake, // has 6 rotations
    }

    // enum AddOn{
    //     Barricade,
    // }

    // enum RiverCrossing{
    //     Ford,
    //     BridgeStone,
    //     BridgeWood
    // }

    #[derive(Clone)]
    pub struct MapTile{
        t_type: BattleMapTileType,
        // add_on: Option<AddOn>,
        // crossing: Option<RiverCrossing>,
    }

    impl MapTile{
        pub fn default() -> Self{
            Self { t_type: BattleMapTileType::Plains }
        }

        pub fn new(t_type: BattleMapTileType) -> Self{
            Self { t_type: t_type }
        }

        pub fn to_console_string(&self) -> String{
            self.t_type.to_console_string()
        }
    }

}


pub mod map{

    use crate::tiles::{CampaignMapTileType, MapTile};


    pub struct Map{
        tiles: Vec<Vec<MapTile>>,
        board_height: usize,
        board_width: usize,
    }

    impl Map{
        pub fn create_map(board_width: usize, board_height: usize, map_tiles: Vec<CampaignMapTileType>) -> Self{
            let mut board = Map::create_empty_board(board_width, board_height);

            Map { tiles: board, board_height, board_width }
        }

        /// Create an empty board based on the widths and heights passed
        fn create_empty_board(width: usize, height: usize) -> Vec<Vec<MapTile>>{
            // must be at least 2x2
            assert!(width >= 2, "Must be at least 2 wide");
            assert!(height >= 2, "Must be at least 2 high");

            // if width is odd, then the even columns will have $height-1$ tiles
            // if width is even, then all columns will have $height$ tiles
            match width % 2 == 0{
                true => { // even width
                    vec![vec![MapTile::default(); width]; height]
                },
                false => { // odd width
                    let mut b = vec![];
                    for i in 0..height{
                        match i % 2 == 0{
                            true => b.push(vec![MapTile::default(); width]),
                            false => b.push(vec![MapTile::default(); width-1])
                        };
                    }
                    b
                }
            }
        }

        /// Print board to console
        pub fn print_board(&self){
            // use /,\,_,| to create board
            println!("Board: {}w x {}h", self.board_width, self.board_height);

            let mut j = 0;
            while j < self.board_height{
                // print top of even row
                print!(" ");
                for _ in 0..self.board_width{
                    print!("/ \\_")
                }
                if self.board_width % 2 == 0{
                    print!("/");
                }
                println!();

                // print even row
                for i in 0..self.board_width{
                    assert!(j % 2 == 0);
                    print!("| {} ", self.tiles[j][i].to_console_string());
                }
                println!("|");
                j += 1;
                
                if j >= self.board_height{
                    break;
                }

                // print bottom of even row
                for _ in 0..self.board_width{
                    print!(" \\_/");
                }
                if self.board_width % 2 == 0{
                    print!(" \\_");
                }
                println!();

                // print odd row
                assert!(j % 2 == 1);
                match self.board_width % 2 == 0{
                    true => { // even -> same length columns
                        print!(" ");
                        for i in 0..self.board_width{ // odd row
                            print!(" | {}", self.tiles[j][i].to_console_string())
                        }
                    },
                    false => { // odd -> different length columns
                        print!(" ");
                        for i in 0..self.board_width-1 { // odd row
                            print!(" | {}", self.tiles[j][i].to_console_string())
                        }
                    }
                }
                println!(" |");
                j += 1;

            }
            
            // print bottom row
            if self.board_height % 2 != 0{
                print!(" \\");
            }
            for _ in 0..self.board_width-1{
                print!(" / \\");
            }
            if self.board_height % 2 == 0{
                print!(" / \\");
            }else{
                print!(" /");
            }
            if (self.board_height % 2 == 0) && (self.board_width % 2 == 0){
                print!(" /");
            }

            println!();
            println!();

        }

        /// Get the neighbors of a given hex
        pub fn get_neighbors(&self, row: usize, column: usize) -> Vec<Option<&MapTile>>{
            // self.tile_exists(row, column);
            
            // top right clockwise to top left
            // top right
            let top_right = match column == self.board_width-1 || row == 0{
                true => None,
                false => {
                    Some(match row % 2 == 0{
                        true => self.get_tile(row-1, column),
                        false => self.get_tile(row-1, column+1)   
                    })
                }
            };

            // right
            let right = match column > 0{
                true => Some(self.get_tile(row, column-1)),
                false => None
            };

            // bottom right
            let bottom_right = match column == self.board_width-1 || row == self.board_height-1{
                true => None,
                false => {
                    Some(match row % 2 == 0{
                        true => self.get_tile(row+1, column),
                        false => self.get_tile(row+1, column+1)   
                    })
                }
            };

            // bottom left
            // let bottom_left = match row % 2 == 0{
            //     true => {
            //         match column == 0{
            //             true => None,
            //             false => Some(self.get_tile(row-1, column-1))
            //         }
            //     },
            //     false => {
            //         match row == self.board_height-1{
            //             true => None,
            //             false => 
            //         }
            //     }
            // };

            // left
            let left = match column < self.board_width-1{
                true => {
                    // if an odd row and odd width, also None, otherwise Some
                    if column % 2 == 1 && self.board_width % 2 != 1{
                        None
                    } else{
                        Some(self.get_tile(row, column+1))
                    }
                }
                false => None
            };

            // top left


            vec![top_right, right, bottom_right, left]
        }

        /// Get a reference to a tile from the board
        pub fn get_tile(&self, row: usize, column: usize) -> &MapTile{
            &self.tiles[row][column]
        }

        fn get_tile_option(&self, row: usize, column: usize) -> Option<&MapTile>{
            match self.tiles.get(row){
                Some(v) => {
                    match v.get(column){
                        Some(s) => Some(s),
                        None => None
                    }
                },
                None => None
            }
        }


    }



}

#[cfg(test)]
mod tests{
    use crate::map::Map;

    #[test]
    fn map_print(){
        for i in 2..=10{
            for j in 2..=10{
                let m = Map::create_map(i,j,vec![]);
                m.print_board();
            }
        }
    }
}