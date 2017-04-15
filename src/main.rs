#[macro_use]
extern crate log;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

mod db;

use db::tomahawk_server::TomahawkServer;

fn main() {
    println!("Hello, world!");
    let server = TomahawkServer::new();
}
