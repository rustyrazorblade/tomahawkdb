

use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::io;

use futures::{future, Future, BoxFuture};
use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_proto::TcpServer;

use super::state::State;

#[derive(Default)]
pub struct GossipCodec;

impl Codec for GossipCodec {
    type In = Message;
    type Out = Message;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        Ok(None)
    }
    fn encode(&mut self, item: Self::Out, into: &mut Vec<u8>) -> io::Result<()> {
        Ok(())
    }

}



pub enum Message {
    // address
    Ping,
    Pong,
    Shutdown,
    AddNode(String),

}

/*
runs the network gossip server and maintains the state of the nodes
should spin off its own thread which uses channels
the instance
*/
pub struct GossipManager {
    nodes: HashMap<String, State>,
}

impl GossipManager {
    /*
    starts the server
    spawns a new thread
    */
    pub fn new() -> GossipManager {
        GossipManager {
            nodes:HashMap::new()
        }
    }

    pub fn get_channel(&self) -> GossipChannel {
        GossipChannel{}
    }
}

//impl Drop for GossipManager {
//    fn drop(&mut self) {
//        self.shutdown();
//    }
//}


pub struct GossipChannel {

}

