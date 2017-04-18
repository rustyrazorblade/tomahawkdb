use super::{GossipError, GossipResult};
use std::collections::HashMap;
use uuid::Uuid;
use bincode::{serialize, deserialize, Infinite};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeState {
    is_up: bool,
    last_update: u64,
}

impl NodeState {
    fn new() -> NodeState {
        NodeState{is_up: false,
            last_update: 0}
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterState {
    nodes: HashMap<String, NodeState>,
}

impl ClusterState {
    fn new() -> ClusterState {
        ClusterState{nodes:HashMap::new()}
    }
    fn update(&mut self, node: Uuid, state: NodeState) -> GossipResult<()> {
        Err(GossipError::OldState)
    }
}