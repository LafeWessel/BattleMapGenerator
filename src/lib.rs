
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
    }

}


mod map{

    use crate::tiles::{CampaignMapTileType, MapTile};


    struct Map{
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
            // if width is odd, then the even columns will have $height-1$ tiles
            // if width is even, then all columns will have $height$ tiles

            match width % 2 == 0{
                true => { // even
                    vec![vec![MapTile::default(); height]; width]
                },
                false => { // odd
                    let mut b = vec![];
                    for i in 0..width{
                        match i % 2 == 0{
                            true => b.push(vec![MapTile::default(); width-1]),
                            false => b.push(vec![MapTile::default(); width])
                        };
                    }
                    b
                }
            }


        }


    }



}