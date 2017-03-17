use std::collections::HashMap;

use super::database::Database;
use super::node::{NodeState, StreamingState, Node};

pub struct TomahawkServer {
    database: Database,
    nodes: HashMap<u8, Node>,
}

impl TomahawkServer {
    pub fn new() -> TomahawkServer {
        TomahawkServer{
            database:Database::new(),
            nodes: HashMap::new() }
    }
}