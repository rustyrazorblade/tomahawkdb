extern crate futures;
//extern crate tokio_core;
//extern crate tokio_io;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
//
//use futures::{Future, Stream};
//use tokio_io::{io, AsyncRead};
//use tokio_core::net::TcpListener;

use super::state::State;

enum Message {

}

/*
runs the network gossip server and maintains the state of the nodes
should spin off its own thread which uses channels
the instance
*/
pub struct GossipServer {
    nodes: HashMap<String, State>,
    process: Option<JoinHandle<()>>,
    // in and out channels
}

impl GossipServer {
    pub fn new() -> GossipServer {
        GossipServer{nodes:HashMap::new(),
            process: None}
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel::<Message>();
        // start it on its own thread
        let process = thread::spawn(|| {

        });
        self.process = Some(process);
    }
    pub fn shutdown(&mut self) {

    }

    pub fn run() {

    }

    pub fn is_running(&self) -> bool {
        match self.process {
            Some(_) => true,
            None => false
        }
    }

    pub fn get_channel(&self) -> GossipChannel {
        GossipChannel{}
    }
}


pub struct GossipChannel {

}

