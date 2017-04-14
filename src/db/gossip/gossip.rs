use std::sync::{Mutex, Arc};

type SharedGossipServer = Arc<Mutex<GossipServer>>;

/*
runs the network gossip server
the shared gossip server should be used
*/
pub struct GossipServer {

}


