
mod map_tiles{
    use colored::{Colorize, ColoredString};


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
        Default
    }

    pub enum CampaignMapTileAddOn{
        Road,
        River,
        RiverFord,
        RiverBridge
    }

    pub struct CampaignMapTile{
        tile: CampaignMapTileType,
        add_on: Option<CampaignMapTileAddOn>,
    }

    impl CampaignMapTile{
        pub fn default() -> Self{
            CampaignMapTile { 
                tile: CampaignMapTileType::Default, 
                add_on: Option::None 
            }
        }
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum TileOwner{
        Defender,
        Attacker,
        LeftFlank,
        RightFlank,
        SplitAttDef
    }

    impl MapPrint for TileOwner {
    
        fn map_print(&self) -> ColoredString{
            match self {
                TileOwner::Defender => String::from("D").cyan(),
                TileOwner::Attacker => String::from("A").red(),
                TileOwner::LeftFlank => String::from("L").green(),
                TileOwner::RightFlank => String::from("R").purple(),
                TileOwner::SplitAttDef => String::from("S").yellow()
            }
        }
    }

    /// Campaign Tiles from which the battle map will be generated
    /// Left and right flank tiles are taken from the defender's perspective
    pub struct CampaignGenerationTiles{
        attacker: CampaignMapTile,
        defender: CampaignMapTile,
        left_flank: CampaignMapTile,
        right_flank: CampaignMapTile,
        cities_within_search_radius: u32, // for calculating population density
        rivers_within_search_radius: u32, // for calculating any streams
        mountains_within_search_radius: u32, // for calculating terrain roughness
        hills_within_search_radius: u32, // for calculating terrain roughness
        search_radius: u32,
    }

    impl CampaignGenerationTiles{
        pub fn new(attacker: CampaignMapTile, defender: CampaignMapTile,
             left_flank: CampaignMapTile, right_flank: CampaignMapTile,
            cities: u32, rivers: u32, mountains: u32, hills: u32, search_radius: u32) -> Self{
                CampaignGenerationTiles{
                    attacker,
                    defender,
                    left_flank,
                    right_flank,
                    cities_within_search_radius: cities,
                    rivers_within_search_radius: rivers,
                    mountains_within_search_radius: mountains,
                    hills_within_search_radius: hills,
                    search_radius
                }
        }

        pub fn default() -> Self{
            CampaignGenerationTiles { 
                attacker: CampaignMapTile::default(),
                defender: CampaignMapTile::default(),
                left_flank: CampaignMapTile::default(),
                right_flank: CampaignMapTile::default(),
                cities_within_search_radius: 0,
                rivers_within_search_radius: 0,
                mountains_within_search_radius: 0,
                hills_within_search_radius: 0,
                search_radius: 1
            }
        }

        /// Calculate the number of hexes within the given radius
        fn radius_search_size(radius: u32) -> u32{
            // formula is 3r^2 + 3r + 1
            3 * (radius * radius) + 3 * radius + 1
        }

        /// number of cities / search area
        pub fn city_density(&self) -> f64{
            self.cities_within_search_radius as f64 / CampaignGenerationTiles::radius_search_size(self.search_radius) as f64
        }

        /// number of cities / search area
        pub fn river_density(&self) -> f64{
            self.rivers_within_search_radius as f64 / CampaignGenerationTiles::radius_search_size(self.search_radius) as f64
        }

        /// number of cities / search area
        pub fn mountain_density(&self) -> f64{
            self.mountains_within_search_radius as f64 / CampaignGenerationTiles::radius_search_size(self.search_radius) as f64
        }

        /// number of cities / search area
        pub fn hill_density(&self) -> f64{
            self.hills_within_search_radius as f64 / CampaignGenerationTiles::radius_search_size(self.search_radius) as f64
        }
    }

    pub trait MapPrint{
        fn map_print(&self) -> ColoredString;
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
        Default,
    }

    impl MapPrint for BattleMapTileType{
        fn map_print(&self) -> ColoredString{
            match self{
                BattleMapTileType::Forest => String::from("F").green(),
                BattleMapTileType::Hill => String::from("H").yellow(),
                BattleMapTileType::Mountain => String::from("M").magenta(),
                BattleMapTileType::Outpost => String::from("O").black().on_white(),
                BattleMapTileType::Plains => String::from("P").blue(),
                BattleMapTileType::River => String::from("R").cyan(),
                BattleMapTileType::Road => String::from("V").black().on_white(),
                BattleMapTileType::Swamp => String::from("S").yellow(),
                BattleMapTileType::Town => String::from("T").black().on_white(),
                BattleMapTileType::Default => String::from("D").white(),
            }
        }
    }

    // enum RoadTileType{
    //     Straight, // has 3 rotations
    //     Turn120, // has 6 rotations
    //     BranchY, // has 2 rotations
    //     Branch120, // has 6 rotations
    // }

    // enum RiverTileType{
    //     Straight, // has 3 rotations
    //     Turn120, // has 6 rotations
    //     BranchY, // has 2 rotations
    //     Branch120, // has 6 rotations
    //     Lake, // has 6 rotations
    // }

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
                t_type: BattleMapTileType::Default,
                owner: TileOwner::Attacker
             }
        }

        pub fn new(t_type: BattleMapTileType) -> Self{
            Self { t_type, owner: TileOwner::Attacker }
        }

        pub fn tile_type_string(&self) -> ColoredString{
            self.t_type.map_print()
        }

        pub fn tile_owner_string(&self) -> ColoredString{
            self.owner.map_print()
        }

        pub fn set_owner(&mut self, owner: TileOwner){
            self.owner = owner;
        }

        pub fn get_owner(&self) -> &TileOwner{
            &self.owner
        }

        pub fn get_type(&self) -> &BattleMapTileType{
            &self.t_type
        }

    }

}


pub mod battle_map{

    use crate::map_tiles::{CampaignMapTileType, MapTile, CampaignGenerationTiles, TileOwner, BattleMapTileType};
    use colored::ColoredString;
    use rand::thread_rng;
    
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

        pub fn default() -> Self{
            MapGenerator{
                base_tiles: CampaignGenerationTiles::default()
            }
        }


        /// Create the default map size (15w x 11h)
        pub fn create_default_map(&self) -> Map{
            self.create_map(15, 11)
        }

        // TODO: add parameters to initialize MapTiles
        pub fn create_map(&self, board_width: usize, board_height: usize) -> Map{
            let mut m = Map { 
                tiles: self.create_empty_board(board_width, board_height), 
                board_height, board_width
                };

                self.set_tile_owners(&mut m);
                self.generate_map_tiles(&mut m);
                m
        }

        /// Create an empty board based on the widths and heights passed
        fn create_empty_board(&self, width: usize, height: usize) -> Vec<Vec<MapTile>>{
            // must be at least 2x2
            assert!(width >= 4, "Must be at least 4 wide");
            assert!(height >= 2, "Must be at least 2 high");

            vec![vec![MapTile::default(); width]; height]
        }

        fn generate_map_tiles(&self, map: &mut Map){
            // calculate total tiles
            let total_tiles: u32 = map.board_height as u32 * map.board_width as u32;

            // determine town count
            let town_ct: u32 = (self.base_tiles.city_density() * total_tiles as f64) as u32;

            // determine river count
            let river_ct: u32 = (self.base_tiles.river_density() * total_tiles as f64) as u32;

            // determine mountain count
            let mtn_ct: u32 = (self.base_tiles.mountain_density() * total_tiles as f64) as u32;

            // determine hill count
            let hill_ct: u32 = (self.base_tiles.hill_density() * total_tiles as f64) as u32;

            // determine which edges any roads should enter/exit on -> WFC?

            // determine which edges any rivers should enter/exit on -> WFC?


            // TODO add better terrain generation
            // randomly place the town, river, mtn, hill tiles
            let mut rng = thread_rng();
            let mut default_tiles: Vec<(usize, usize)> = (0..map.board_height).into_iter().map(|w| (0..map.board_width).into_iter().map(|h| (w,h)).collect::<Vec<(usize, usize)>>()).flatten().collect();
            for _ in 0..hill_ct{
                
            }
            for _ in 0..town_ct{

            }
            for _ in 0..mtn_ct{

            }
            for _ in 0..river_ct{

            }


        }

        /// finds a random tile on the map that is the Default type, panics if there are none
        fn find_default_tile(map: &Map, default_tiles: &Vec<(usize, usize)>) -> (usize, usize){
            (0, 0)
        }

        fn set_tile_owners(&self, map: &mut Map){
            let flank_width = map.board_width / 4;
            let vertical_owner_depth = map.board_height / 2;
            
            // set flank owners
            for v in map.tiles.iter_mut(){
                for t in v.iter_mut().take(flank_width){
                    t.set_owner(TileOwner::LeftFlank);
                }

                for t in v.iter_mut().skip(map.board_width - flank_width){
                    t.set_owner(TileOwner::RightFlank);
                }
            }

            // set attacker owners
            for v in map.tiles.iter_mut().take(vertical_owner_depth){
                for t in v.iter_mut().skip(flank_width).take(map.board_width - (2 * flank_width)){
                    t.set_owner(TileOwner::Attacker);
                }
            }

            // set defender owners
            for v in map.tiles.iter_mut().skip(map.board_height - vertical_owner_depth){
                for t in v.iter_mut().skip(flank_width).take(map.board_width - (2 * flank_width)){
                    t.set_owner(TileOwner::Defender);
                }
            }

            // set split owners if odd height
            if map.board_height % 2 == 1{
                for t in  map.tiles[vertical_owner_depth].iter_mut().skip(flank_width).take(map.board_width - (2 * flank_width)){
                    t.set_owner(TileOwner::SplitAttDef);
                }
            }


        }





    }



    pub struct Map{
        tiles: Vec<Vec<MapTile>>,
        board_height: usize,
        board_width: usize,
    }

    impl Map{
        pub fn print_board_tiles(&self){
            self.print_board(&self.tiles.iter().map(|v| v.iter().map(|i| i.tile_type_string()).collect()).collect())
        }

        pub fn print_board_owners(&self){
            self.print_board(&self.tiles.iter().map(|v| v.iter().map(|i| i.tile_owner_string()).collect()).collect())
        }

        /// Print board to console
        fn print_board(&self, tile_abbrs: &Vec<Vec<ColoredString>>){
            // use /,\,_,| to create board
            println!("Board: {}w x {}h", self.board_width, self.board_height);

            let mut j = 0;
            while j < self.board_height{
                // print top of even row
                print!(" ");
                for _ in 0..self.board_width{
                    print!("/ \\_")
                }
                println!("/");

                // print even row
                for i in 0..self.board_width{
                    assert!(j % 2 == 0);
                    print!("| {} ", tile_abbrs[j][i]);
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
                println!(" \\");

                // print odd row
                assert!(j % 2 == 1);
                print!(" ");
                for i in 0..self.board_width{ // odd row
                    print!(" | {}", tile_abbrs[j][i])
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
            print!(" / \\");
            if self.board_height % 2 == 0{
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
        pub fn get_tile(&self, row: usize, column: usize) -> Option<&MapTile>{
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
    use crate::{battle_map::{Map, MapGenerator}, map_tiles::{BattleMapTileType, MapTile, TileOwner}};

    #[test]
    fn map_print(){
        for i in 4..=10{
            for j in 2..=10{
                let m = MapGenerator::default().create_map(i,j);
                m.print_board_tiles();
            }
        }
    }

    #[test]
    fn get_neighbors(){
        // Arrange
        let own = TileOwner::Attacker;
        let mut m = MapGenerator::default().create_map(4, 3);
        m.set_tile(0, 0, MapTile::new(BattleMapTileType::Plains));
        m.set_tile(0, 1, MapTile::new(BattleMapTileType::Forest));
        m.set_tile(0, 2, MapTile::new(BattleMapTileType::Hill));
        m.set_tile(1, 0, MapTile::new(BattleMapTileType::Mountain));
        m.set_tile(1, 1, MapTile::new(BattleMapTileType::Outpost));
        m.set_tile(2, 0, MapTile::new(BattleMapTileType::Road));
        m.set_tile(2, 1, MapTile::new(BattleMapTileType::Swamp));
        m.set_tile(2, 2, MapTile::new(BattleMapTileType::Town));
        
        // Act
        let neighbors_0_0 = m.get_neighbors(0, 0);
        let neighbors_1_1 = m.get_neighbors(1, 1);
        let neighbors_2_1 = m.get_neighbors(2, 1);

        // Assert
        assert_eq!(neighbors_0_0.get_tile_location(), (0,0));
        assert_eq!(neighbors_0_0.tile_type().get_type(), &BattleMapTileType::Plains);
        assert_eq!(neighbors_0_0.get_left(), None);
        assert_eq!(neighbors_0_0.get_upper_left(), None);
        assert_eq!(neighbors_0_0.get_upper_right(), None);
        assert_eq!(neighbors_0_0.get_right().unwrap().get_type(), &BattleMapTileType::Forest);
        assert_eq!(neighbors_0_0.get_lower_right().unwrap().get_type(), &BattleMapTileType::Mountain);
        assert_eq!(neighbors_0_0.get_lower_left(), None);
        
        assert_eq!(neighbors_1_1.get_tile_location(), (1,1));
        assert_eq!(neighbors_1_1.tile_type().get_type(), &BattleMapTileType::Outpost);
        assert_eq!(neighbors_1_1.get_left().unwrap().get_type(), &BattleMapTileType::Mountain);
        assert_eq!(neighbors_1_1.get_upper_left().unwrap().get_type(), &BattleMapTileType::Forest);
        assert_eq!(neighbors_1_1.get_upper_right().unwrap().get_type(), &BattleMapTileType::Hill);
        assert_eq!(neighbors_1_1.get_right().unwrap().get_type(), &BattleMapTileType::Default);
        assert_eq!(neighbors_1_1.get_lower_right().unwrap().get_type(), &BattleMapTileType::Town);
        assert_eq!(neighbors_1_1.get_lower_left().unwrap().get_type(), &BattleMapTileType::Swamp);
        
        assert_eq!(neighbors_2_1.get_tile_location(), (2,1));
        assert_eq!(neighbors_2_1.tile_type().get_type(), &BattleMapTileType::Swamp);
        assert_eq!(neighbors_2_1.get_left().unwrap().get_type(), &BattleMapTileType::Road);
        assert_eq!(neighbors_2_1.get_upper_left().unwrap().get_type(), &BattleMapTileType::Mountain);
        assert_eq!(neighbors_2_1.get_upper_right().unwrap().get_type(), &BattleMapTileType::Outpost);
        assert_eq!(neighbors_2_1.get_right().unwrap().get_type(), &BattleMapTileType::Town);
        assert_eq!(neighbors_2_1.get_lower_right(), None);
        assert_eq!(neighbors_2_1.get_lower_left(), None);
    }


    #[test]
    fn tile_owners(){

        // even width, even height
        let mut m = MapGenerator::default().create_map(4, 2);
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(0, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(0, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(1, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(1, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(1, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(1, 3).unwrap().get_owner());

        // even width, odd height
        let mut m = MapGenerator::default().create_map(4, 3);
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(0, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(0, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(1, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::SplitAttDef, m.get_tile(1, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::SplitAttDef, m.get_tile(1, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(1, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(2, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(2, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(2, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(2, 3).unwrap().get_owner());
        
        // odd width, even height
        let mut m = MapGenerator::default().create_map(5, 2);
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(0, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(0, 4).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(1, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(1, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(1, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(1, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(1, 4).unwrap().get_owner());

        // odd width, odd height
        let mut m = MapGenerator::default().create_map(5, 3);
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(0, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::Attacker, m.get_tile(0, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(0, 4).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(1, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::SplitAttDef, m.get_tile(1, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::SplitAttDef, m.get_tile(1, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::SplitAttDef, m.get_tile(1, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(1, 4).unwrap().get_owner());
        assert_eq!(&TileOwner::LeftFlank, m.get_tile(2, 0).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(2, 1).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(2, 2).unwrap().get_owner());
        assert_eq!(&TileOwner::Defender, m.get_tile(2, 3).unwrap().get_owner());
        assert_eq!(&TileOwner::RightFlank, m.get_tile(2, 4).unwrap().get_owner());
    }
}