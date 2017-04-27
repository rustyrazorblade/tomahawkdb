use super::{GossipError, GossipResult};
use std::collections::HashMap;
use uuid::Uuid;
use bincode::{serialize, deserialize, Infinite};
use std::cmp::{PartialOrd, Ordering};
use std::net::Ipv4Addr;
use super::gossip::Message;

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
    id: Uuid,
    address: Ipv4Addr,
    port: usize,
    state: HashMap<ApplicationState, VersionedValue>,
}

impl NodeState {
    fn new(ip: Ipv4Addr, port: usize) -> NodeState {
        let id = Uuid::new_v4();

        NodeState{is_up: false,
            id: id,
            address: ip,
            port: port,
            state: HashMap::new()
            }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterState {
    nodes: Vec<NodeState>,
}

impl ClusterState {
    pub fn new() -> ClusterState {
        ClusterState{nodes:Vec::new()}
    }

    pub fn handle(&mut self, message: Message) -> Message {
        Message::ReceivedOK
    }

    fn update(&mut self, node: Uuid, state: NodeState) -> GossipResult<()> {
        Err(GossipError::OldState)
    }

}