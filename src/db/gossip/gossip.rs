use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use super::state::State;



/*
runs the network gossip server and maintains the state of the nodes
should spin off its own thread which uses channels
the instance
*/
pub struct GossipServer {
    nodes: HashMap<String, State>,
    // in and out channels
}

impl GossipServer {
    pub fn new() -> GossipServer {
        GossipServer{nodes:HashMap::new()}
    }

    pub fn start(&mut self) {

    }
    pub fn shutdown(&mut self) {

    }
    pub fn is_running(&self) -> bool {
        false
    }

    pub fn get_channel(&self) -> GossipChannel {
        GossipChannel{}
    }
}


pub struct GossipChannel {

}

