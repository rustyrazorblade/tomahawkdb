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

use super::MessagingCodec;

#[derive(Debug)]
pub struct MessagingProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for MessagingProto {
    type Request = Message;
    type Response = Message;
    type Transport = Framed<T, MessagingCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MessagingCodec))
    }
}




#[derive(Debug)]
pub struct MessagingClientProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for MessagingClientProto {
    type Request = Message;
    type Response = Message;
    type Transport = Framed<T, MessagingCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MessagingCodec))
    }
}
