use std::collections::BTreeMap;

use island::IslandFactory;
use protocol::{Cell, Clue, IslandId, MapError, Params, Seed, Terrain};

///To build a Map by hand and not from the seed. Can be useful to testing and debugging
pub struct ScenarioBuilder {
    params: Params,
    terrain: BTreeMap<Cell, Terrain>,
    treasure: Option<Cell>,
    weather_seed: Seed,
    islands: BTreeMap<IslandId, (Clue, IslandFactory)>,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum ScenarioError {
    RadiusNotAboveVision,
    Map(MapError),
    UndeclaredIsland(IslandId),
    CountMismatch,
    BadLand,
    BadTreasure,
    WaterNotConnected,
    Unsound(IslandId),
    Insufficient(usize),
}
