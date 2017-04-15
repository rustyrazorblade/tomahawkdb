
// Standard libs
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::io;
use std::io::Cursor; // used with byteorder

// TOKIO
use futures::{future, Future, BoxFuture};
use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_proto::TcpServer;

// Serialization
use bincode::{serialize, deserialize, Infinite};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};


// My stuff
use super::state::State;

#[derive(Default)]
pub struct GossipCodec;

// Lets us work with the Message enum at the higher level.
impl Codec for GossipCodec {
    type In = Message;
    type Out = Message;

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<Self::In>, io::Error> {
        // read the length
        // read the data, get a message
        let mut tmp = Cursor::new(buf.drain_to(4));
        let lenth = tmp.read_u32::<BigEndian>()?;
        let data = buf.drain_to(lenth as usize);
        let message = deserialize::<Message>(data.as_slice()).
            map_err(|x| io::Error::new(io::ErrorKind::Other, "Could not deserialize"))?;
        Ok(Some(message))
    }

    fn encode(&mut self, item: Self::Out, into: &mut Vec<u8>) -> io::Result<()> {
        // serialize the Message
        // write the length to the vector and then the data
        let mut data = serialize(&item, Infinite).
            map_err(|x| io::Error::new(io::ErrorKind::Other, "could not serialize") )?;
        let mut len = data.len() as u32;
        into.write_u32::<BigEndian>(len);
        into.append(&mut data);
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
        GossipManager { nodes: HashMap::new() }
    }

    pub fn get_channel(&self) -> GossipChannel {
        GossipChannel {}
    }
}

//impl Drop for GossipManager {
//    fn drop(&mut self) {
//        self.shutdown();
//    }
//}


pub struct GossipChannel {}
