use super::gossip::GossipServer;


#[test]
fn start_and_stop_gossip() {
    let mut g = GossipServer::new();
    g.start();
    assert!(g.is_running());
    let chan = g.get_channel();
    g.shutdown();
}

#[test]
fn add_server() {

}