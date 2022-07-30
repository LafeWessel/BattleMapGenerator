
mod map_tiles{

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

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum TileOwner{
        Defender,
        Attacker,
        LeftFlank,
        RightFlank,
        SplitAttDef
    }

    impl TileOwner {
    
        pub fn to_console_string(&self) -> String{
            String::from(match &self{
                Defender => "D",
                Attacker => "A",
                LeftFlank => "L",
                RightFlank => "R",
                SplitAttDef => "S"
            })
        }
    }

    /// Campaign Tiles from which the battle map will be generated
    /// Left and right flank tiles are taken from the attacker's perspective
    pub struct CampaignGenerationTiles{
        attacker_tile: CampaignMapTileType,
        defender_tile: CampaignMapTileType,
        left_flank_tile: CampaignMapTileType,
        right_flank_tile: CampaignMapTileType
    }

    impl CampaignGenerationTiles{
        pub fn new(attacker_tile: CampaignMapTileType, defender_tile: CampaignMapTileType,
             left_flank_tile: CampaignMapTileType, right_flank_tile: CampaignMapTileType) -> Self{
                CampaignGenerationTiles{
                    attacker_tile,
                    defender_tile,
                    left_flank_tile,
                    right_flank_tile
                }
        }
    }

    // TODO add any missing tile types that are in the base game
    #[derive(Clone, PartialEq, Eq, Debug)]
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

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct MapTile{
        t_type: BattleMapTileType,
        owner: TileOwner
        // add_on: Option<AddOn>,
        // crossing: Option<RiverCrossing>,
    }

    impl MapTile{
        pub fn default() -> Self{
            Self { 
                t_type: BattleMapTileType::Plains,
                owner: TileOwner::Attacker
             }
        }

        pub fn new(t_type: BattleMapTileType, owner: TileOwner) -> Self{
            Self { t_type, owner }
        }

        pub fn to_console_string(&self) -> String{
            self.t_type.to_console_string()
        }

        pub fn to_owner_string(&self) -> String{
            self.owner.to_console_string()
        }
    }

}


pub mod battle_map{

    use crate::map_tiles::{CampaignMapTileType, MapTile, CampaignGenerationTiles};
    
    pub struct TileNeighbors<'a>{
        tile_location: (usize, usize),
        tile: &'a MapTile,
        left: Option<&'a MapTile>,
        upper_left: Option<&'a MapTile>,
        upper_right: Option<&'a MapTile>,
        right: Option<&'a MapTile>,
        lower_right: Option<&'a MapTile>,
        lower_left: Option<&'a MapTile>
    }

    impl<'a> TileNeighbors<'a>{
        fn new(tile_location: (usize, usize), tile: &'a MapTile, 
        left: Option<&'a MapTile>, upper_left: Option<&'a MapTile>, upper_right: Option<&'a MapTile>, 
        right: Option<&'a MapTile>, lower_right: Option<&'a MapTile>, lower_left: Option<&'a MapTile>) -> Self{
            TileNeighbors { 
                tile_location, tile,
                left, upper_left, 
                upper_right, right, 
                lower_right, lower_left
             }
        }

        pub fn get_tile_location(&self) -> (usize, usize){
            self.tile_location
        }

        pub fn tile_type(&self) -> &MapTile{
            self.tile
        }

        pub fn get_lower_left(&self) -> Option<&'a MapTile>{
            self.lower_left
        }      

        pub fn get_left(&self) -> Option<&'a MapTile>{
            self.left
        }

        pub fn get_upper_left(&self) -> Option<&'a MapTile>{
            self.upper_left
        }

        pub fn get_upper_right(&self) -> Option<&'a MapTile>{
            self.upper_right
        }

        pub fn get_right(&self) -> Option<&'a MapTile>{
            self.right
        }

        pub fn get_lower_right(&self) -> Option<&'a MapTile>{
            self.lower_right
        }


    }

    pub struct MapGenerator{
        base_tiles: CampaignGenerationTiles
    }

    impl MapGenerator
    {
        pub fn new(base_tiles: CampaignGenerationTiles) -> Self {
            MapGenerator{
                base_tiles
            }
        }
    }



    pub struct Map{
        tiles: Vec<Vec<MapTile>>,
        board_height: usize,
        board_width: usize,
    }

    impl Map{

        /// Create the default map size (15w x 11h)
        pub fn create_default_map() -> Self{
            Map::create_map(15, 11)
        }

        // TODO: add parameters to initialize MapTiles
        pub fn create_map(board_width: usize, board_height: usize) -> Self{
            Map { 
                tiles: Map::create_empty_board(board_width, board_height), 
                board_height, board_width
             }
        }

        /// Create an empty board based on the widths and heights passed
        fn create_empty_board(width: usize, height: usize) -> Vec<Vec<MapTile>>{
            // must be at least 2x2
            assert!(width >= 4, "Must be at least 4 wide");
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
        pub fn get_neighbors(&self, row: usize, column: usize) -> TileNeighbors{

            let tile = self.get_tile(row, column).expect("Tile must exist to get neighbors");
            
            // right
            let right = self.get_tile(row, column+1);
            // left
            let left = match column == 0 {
                true => None,
                false => self.get_tile(row, column-1)
            };

            // top right clockwise to top left
            let (upper_left, upper_right, lower_right, lower_left) = match row % 2 == 0{
                true => {
                    // upper left
                    let ul = match row == 0 || column == 0 { 
                        true => None,
                        false => self.get_tile(row-1, column-1)
                    };
                    // upper right
                    let ur = match row == 0 {
                        true => None,
                        false => self.get_tile(row-1, column)
                    };
                    // lower right
                    let lr = self.get_tile(row+1, column);
                    // lower left
                    let ll = match column == 0 {
                        true => None,
                        false => self.get_tile(row+1, column-1)
                    };
                    (ul, ur, lr, ll)
                },
                false => {
                    // upper left
                    let ul = match row == 0{
                        true => None,
                        false => self.get_tile(row-1, column)
                    };
                    // upper right
                    let ur = match row == 0{
                        true => None,
                        false => self.get_tile(row-1, column+1)
                    };
                    // lower right
                    let lr = self.get_tile(row+1, column+1);
                    // lower left
                    let ll = self.get_tile(row+1, column);
                    (ul, ur, lr, ll)
                }
            };
            
            TileNeighbors::new((row, column), tile, left, upper_left, upper_right, right, lower_right, lower_left)

        }

        /// Get a reference to a tile from the board
        fn get_tile(&self, row: usize, column: usize) -> Option<&MapTile>{
            match self.tiles.get(row){
                Some(v) => {
                    v.get(column)
                },
                None => None
            }
        }

        /// Set a tile
        pub fn set_tile(&mut self, row: usize, column: usize, tile: MapTile){
            self.get_tile(row, column).expect(&format!("Tile must exist to set, ({},{})", row, column));
            self.tiles[row][column] = tile;
        }


    }



}

#[cfg(test)]
mod tests{
    use crate::{battle_map::Map, map_tiles::{BattleMapTileType, MapTile, TileOwner}};

    #[test]
    fn map_print(){
        for i in 2..=10{
            for j in 2..=10{
                let m = Map::create_map(i,j);
                m.print_board();
            }
        }
    }

    #[test]
    fn get_neighbors(){
        // Arrange
        let own = TileOwner::Attacker;
        let mut m = Map::create_map(3, 3);
        m.set_tile(0, 0, MapTile::new(BattleMapTileType::Plains, own.clone()));
        m.set_tile(0, 1, MapTile::new(BattleMapTileType::Forest, own.clone()));
        m.set_tile(0, 2, MapTile::new(BattleMapTileType::Hill, own.clone()));
        m.set_tile(1, 0, MapTile::new(BattleMapTileType::Mountain, own.clone()));
        m.set_tile(1, 1, MapTile::new(BattleMapTileType::Outpost, own.clone()));
        m.set_tile(2, 0, MapTile::new(BattleMapTileType::Road, own.clone()));
        m.set_tile(2, 1, MapTile::new(BattleMapTileType::Swamp, own.clone()));
        m.set_tile(2, 2, MapTile::new(BattleMapTileType::Town, own.clone()));
        
        // Act
        let neighbors_0_0 = m.get_neighbors(0, 0);
        let neighbors_1_1 = m.get_neighbors(1, 1);
        let neighbors_2_1 = m.get_neighbors(2, 1);

        // Assert
        assert_eq!(neighbors_0_0.get_tile_location(), (0,0));
        assert_eq!(neighbors_0_0.tile_type(), &MapTile::new(BattleMapTileType::Plains, own.clone()));
        assert_eq!(neighbors_0_0.get_left(), None);
        assert_eq!(neighbors_0_0.get_upper_left(), None);
        assert_eq!(neighbors_0_0.get_upper_right(), None);
        assert_eq!(neighbors_0_0.get_right(), Some(&MapTile::new(BattleMapTileType::Forest, own.clone())));
        assert_eq!(neighbors_0_0.get_lower_right(), Some(&MapTile::new(BattleMapTileType::Mountain, own.clone())));
        assert_eq!(neighbors_0_0.get_lower_left(), None);
        
        assert_eq!(neighbors_1_1.get_tile_location(), (1,1));
        assert_eq!(neighbors_1_1.tile_type(), &MapTile::new(BattleMapTileType::Outpost, own.clone()));
        assert_eq!(neighbors_1_1.get_left(), Some(&MapTile::new(BattleMapTileType::Mountain, own.clone())));
        assert_eq!(neighbors_1_1.get_upper_left(), Some(&MapTile::new(BattleMapTileType::Forest, own.clone())));
        assert_eq!(neighbors_1_1.get_upper_right(), Some(&MapTile::new(BattleMapTileType::Hill, own.clone())));
        assert_eq!(neighbors_1_1.get_right(), None);
        assert_eq!(neighbors_1_1.get_lower_right(), Some(&MapTile::new(BattleMapTileType::Town, own.clone())));
        assert_eq!(neighbors_1_1.get_lower_left(), Some(&MapTile::new(BattleMapTileType::Swamp, own.clone())));
        
        assert_eq!(neighbors_2_1.get_tile_location(), (2,1));
        assert_eq!(neighbors_2_1.tile_type(), &MapTile::new(BattleMapTileType::Swamp, own.clone()));
        assert_eq!(neighbors_2_1.get_left(), Some(&MapTile::new(BattleMapTileType::Road, own.clone())));
        assert_eq!(neighbors_2_1.get_upper_left(), Some(&MapTile::new(BattleMapTileType::Mountain, own.clone())));
        assert_eq!(neighbors_2_1.get_upper_right(), Some(&MapTile::new(BattleMapTileType::Outpost, own.clone())));
        assert_eq!(neighbors_2_1.get_right(), Some(&MapTile::new(BattleMapTileType::Town, own.clone())));
        assert_eq!(neighbors_2_1.get_lower_right(), None);
        assert_eq!(neighbors_2_1.get_lower_left(), None);
    }


    #[test]
    fn tile_owners(){

    }
}