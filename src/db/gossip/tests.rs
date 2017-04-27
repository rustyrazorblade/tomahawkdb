//use super::GossipManager;
use uuid::Uuid;
use super::{GossipService, ClusterState, Message};
/*
lets be sure the cluster manager works

*/


#[test]
fn start_and_stop_gossip() {
//    let mut g = GossipManager::new();
}

#[test]
fn ping_pong_channel() {
    GossipService::new_async(48475);
}

#[test]
fn add_node() {
    let mut state = ClusterState::new();
    assert_eq!(state.node_count(), 0);
    let msg = Message::Join(Uuid::new_v4(),
                            String::from("127.0.0.1"),
                            8375);
    let result = state.handle(msg);
    assert_eq!(state.node_count(), 1);

}




