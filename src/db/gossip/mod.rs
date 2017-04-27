pub mod gossip;
pub mod messages;
pub mod state;

use self::state::{ClusterState, NodeState};
pub use self::gossip::GossipService;

mod tests;

enum GossipError {
    OldState, // returned when we try to update the state with one that's older
}

type GossipResult<T> = Result<T, GossipError>;
