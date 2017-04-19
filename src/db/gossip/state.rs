use super::{GossipError, GossipResult};
use std::collections::HashMap;
use uuid::Uuid;
use bincode::{serialize, deserialize, Infinite};



#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Eq, PartialOrd)]
enum ApplicationState {
    Status,
    Load,
    Schema,
    Datacenter,
    Rack,
    Version,
    InternalIp,
    HostId,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeState {
    is_up: bool,
    version: u64,
    state: HashMap<ApplicationState, String>,
}

impl NodeState {
    fn new() -> NodeState {
        NodeState{is_up: false,
            version: 0,
            state: HashMap::new()}
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