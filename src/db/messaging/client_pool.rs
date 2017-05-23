use std::collections::HashMap;
use uuid::Uuid;
//use tokio_proto::TcpClient;
use tokio_core::net::TcpStream;


/// Client pool is used to maintain a single client id : socket HashMap
/// It also maintains a dedicated Handle
struct ClientPool {

    pool: HashMap<Uuid, TcpStream>
}

impl ClientPool {

}