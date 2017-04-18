pub mod gossip;
pub mod messages;
pub mod state;

use self::state::{ClusterState, NodeState};

mod tests;

enum GossipError {
    OldState, // returned when we try to update the state with one that's older
}

type GossipResult<T> = Result<T, GossipError>;
