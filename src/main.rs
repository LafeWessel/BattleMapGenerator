use map_generator::battle_map::{MapGenerator};
use map_generator::map_tiles::{CampaignGenerationTiles, CampaignMapTile};

fn main() {

    for i in 4..=5{
        for j in 4..=5{
            let m = MapGenerator::new(CampaignGenerationTiles::new(
                CampaignMapTile::default(), 
                CampaignMapTile::default(), 
                CampaignMapTile::default(), 
                CampaignMapTile::default(), 
                1, 1, 1, 1, 1,))
                .create_map(i,j);
            m.print_board_tiles();
        }
    }

    MapGenerator::default().create_default_map().print_board_tiles();

    // let m = Map::create_map(4, 6, vec![]);
    // m.print_board();


    for i in 4..=5{
        for j in 2..=5{
            let m = MapGenerator::default().create_map(i,j);
            m.print_board_owners();
        }
    }

    MapGenerator::default().create_default_map().print_board_owners();

}
