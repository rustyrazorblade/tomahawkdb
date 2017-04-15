use std::collections::HashMap;


pub struct ClusterState {
    nodes: HashMap<String, State>,
}

impl ClusterState {}

// keeps the state of each node
// it will even track itself!
pub struct State {
    // sockets
    values: HashMap<String, String>,
}

static UP: &'static str = "UP";
