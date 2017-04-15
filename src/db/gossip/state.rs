use std::collections::HashMap;

pub struct NodeState {
    is_up: bool,
    last_update: u64,
}

impl NodeState {
    fn new() -> NodeState {
        NodeState{is_up: false,
            last_update: 0}
    }
}

