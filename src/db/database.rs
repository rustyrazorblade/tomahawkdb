use std::collections::HashMap;

type ByteBuffer = Vec<u8>;


pub struct Database {
    data: HashMap<ByteBuffer, ByteBuffer>,
}



impl Database {
    pub fn new() -> Database {
        Database{data: HashMap::new()}
    }
}