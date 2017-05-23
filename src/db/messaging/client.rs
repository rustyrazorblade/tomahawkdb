// TOKIO
use tokio_proto::pipeline::ClientService;
use tokio_core::net::TcpStream;

use super::proto::MessagingClientProto;

pub struct Client;

pub struct ClientHandle {
    inner: ClientService<TcpStream, MessagingClientProto>,
}
