#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

extern crate byteorder;
extern crate bytes;

extern crate uuid;

mod db;

use db::tomahawk_server::TomahawkServer;
use db::gossip::GossipService;

fn main() {
    println!("Hello, world!");
    let server = TomahawkServer::new();
    GossipService::new(12345);
}
