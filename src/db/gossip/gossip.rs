
// Standard libs
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::io;
use std::io::Cursor; // used with byteorder
use bytes::{BytesMut, BufMut}; // needed for tokio
use std::sync::RwLock;
use std::net::SocketAddr;

// TOKIO
use futures::{future, Future, BoxFuture, finished};
use tokio_proto::pipeline::ServerProto;
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::TcpServer;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_service::Service;


// Serialization
use bincode::{serialize, deserialize, Infinite};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};


// My stuff
use super::state::ClusterState;

#[derive(Default)]
pub struct GossipCodec;

// Lets us work with the Message enum at the higher level.
impl Decoder for GossipCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        // read the length
        // read the data, get a message
        let mut tmp = Cursor::new(buf.drain_to(4));
        let lenth = tmp.read_u32::<BigEndian>()?;
        let data = buf.drain_to(lenth as usize);
        let message = deserialize::<Message>(&*data).
            map_err(|x| io::Error::new(io::ErrorKind::Other, "Could not deserialize"))?;
        Ok(Some(message))
    }
}

impl Encoder for GossipCodec {
    type Item = Message;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, into: &mut BytesMut) -> io::Result<()> {
        // serialize the Message
        // write the length to the vector and then the data
        let mut data = serialize(&item, Infinite).
        map_err(|x| io::Error::new(io::ErrorKind::Other, "could not serialize") )?;
        let mut len = data.len() as u32;
        into.put_u32::<BigEndian>(len);
        into.put_slice(&mut data);
        Ok(())
    }
}


#[derive(Debug)]
pub struct GossipProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for GossipProto {
    type Request = Message;
    type Response = Message;
    type Transport = Framed<T, GossipCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(GossipCodec))
    }
}

pub struct GossipService {
    address: SocketAddr,
    state: RwLock<ClusterState>,
}

impl GossipService {
    pub fn new(port: usize)  {
        let address = format!("0.0.0.0:{}", port);
        let addr = address.parse().unwrap();

        let server = TcpServer::new(GossipProto, addr);
        server.serve(move ||
            Ok(GossipService{
                address: addr,
                state: RwLock::new(ClusterState::new())
            }));
    }
}

impl Service for GossipService {
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;
    type Request = Message;
    type Response = Message;

    fn call(&self, req: Self::Request) -> Self::Future {
        let tmp = self.state.write().unwrap();

        finished(Message::ReceivedOK).boxed()
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Message {
    // testing only
    Ping,
    Pong,
    // normal response
    ReceivedOK,
    Shutdown,
    // address
    GossipMessage(ClusterState),
}


