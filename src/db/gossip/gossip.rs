
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
use uuid::Uuid;
// TOKIO
use futures::{future, Future, BoxFuture, finished};
use tokio_proto::pipeline::{ServerProto, ClientProto};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::{TcpServer, TcpClient};
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


#[derive(Debug)]
pub struct GossipClientProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for GossipClientProto {
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
    // starts a new gossip server on a thread
    pub fn new_async(port: usize) -> thread::JoinHandle<()> {
        thread::spawn(move|| GossipService::new(port))
    }
}

impl Service for GossipService {
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;
    type Request = Message;
    type Response = Message;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut tmp = self.state.write().unwrap();
        let response = tmp.handle(req);
        finished(Message::ReceivedOK).boxed()
    }

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Message {
    // testing only
    Ping,
    Pong,
    // normal response
    // server has its own UUID, ip & port
    Join(Uuid, String, usize),
    ReceivedOK,
    Shutdown,
    // address
    GossipMessage(ClusterState),
}

struct GossipClient {
    // presumably keeps the connection here
}

impl GossipClient {
    fn new(address: SocketAddr) -> GossipClient {
        let tcp = TcpClient::new(GossipClientProto);
        GossipClient{}
    }
}

