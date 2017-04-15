#[macro_use]
extern crate log;

mod db;

use db::tomahawk_server::TomahawkServer;

fn main() {
    println!("Hello, world!");
    let server = TomahawkServer::new();
}
