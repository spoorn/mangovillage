use bevy::prelude::Resource;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use crate::common::components::Position;

#[derive(Resource, Default, Debug)]
pub struct World {
    // Level iid -> Map
    pub maps: HashMap<String, Map>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Map {
    // x1, x2, y1, y2 bounds, post Transform
    pub bounds: [f32; 4],
    // (Direction e.g. 'n', 's', 'e', 'w', Neighbor iid)
    pub neighbors: Vec<(String, String)>,
    // Player spawn position
    pub player_spawn: Position,
    // (Portal [x1, x2, y1, y2] ranges, destination Level iid)
    pub portals: Vec<PortalInfo>
}

// (Portal [x1, x2, y1, y2] ranges, destination Level iid, link ID)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PortalInfo(pub [f32; 4], pub String, pub i32);