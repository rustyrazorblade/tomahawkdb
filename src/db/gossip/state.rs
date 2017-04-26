use super::{GossipError, GossipResult};
use std::collections::HashMap;
use uuid::Uuid;
use bincode::{serialize, deserialize, Infinite};
use std::cmp::{PartialOrd, Ordering};



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
struct VersionedValue {
    version: i64,
    value: String,
}

impl PartialOrd for VersionedValue {
    fn partial_cmp(&self, other: &VersionedValue) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeState {
    is_up: bool,
    state: HashMap<ApplicationState, VersionedValue>,
}

impl NodeState {
    fn new() -> NodeState {
        NodeState{is_up: false,
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