use std::net::Ipv4Addr;

pub enum NodeState {
    Up,
    Down,
}

pub enum StreamingState {
    Joining,
    Normal,
}

pub struct Node {
    address: Ipv4Addr,
    state: NodeState,
    streaming_state: StreamingState,

}