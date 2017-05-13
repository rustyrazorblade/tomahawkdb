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

use super::Message;

// Serialization
use bincode::{serialize, deserialize, Infinite};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};


#[derive(Default)]
pub struct MessagingCodec;

// Lets us work with the Message enum at the higher level.
impl Decoder for MessagingCodec {
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

impl Encoder for MessagingCodec {
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