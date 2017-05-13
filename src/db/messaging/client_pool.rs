/// Client pool is used to maintain a single client id : socket HashMap
/// It also maintains a dedicated Handle
struct ClientPool {
    pool: HashMap<Uuid, TcpClient>
}