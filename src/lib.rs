
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

    struct MapTile{
        b_type: BattleMapTileType,
        // add_on: Option<AddOn>,
        // crossing: Option<RiverCrossing>,
    }

}