
mod tiles{

    enum CampaignMapTileType{
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
    enum BattleMapTileType{
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

    // enum RoadTileTypes{

    // }

    // enum RiverTileTypes{

    // }

    // enum AddOn{
    //     Barricade,
    // }

    // enum RiverCrossing{
    //     Ford,
    //     BridgeStone,
    //     BridgeWood
    // }

    struct MapTile{
        b_type: BattleMapTileType,
        // add_on: Option<AddOn>,
        // crossing: Option<RiverCrossing>,
    }

}