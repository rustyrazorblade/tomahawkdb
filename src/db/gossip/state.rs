use std::collections::HashMap;
// keeps the state of each node
// it will even track itself!
pub struct State {
    // sockets
    values: HashMap<String, String>,
}

static UP : &'static str = "UP";