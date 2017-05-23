pub mod gossip;
pub mod state;

pub use self::state::{ClusterState, NodeState};
pub use self::gossip::{GossipService, Message};

mod tests;

enum GossipError {
    OldState, // returned when we try to update the state with one that's older
}

type GossipResult<T> = Result<T, GossipError>;
