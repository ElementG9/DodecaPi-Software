// use std::io::prelude::*;
// use std::net::{TcpListener, TcpStream};
pub mod config;
pub mod network;

pub fn start(config_filename: &str) {
    let conf = config::read_config(config_filename).unwrap();
    let listener = network::start_listener(conf.portin)
        .expect(&format!("Could not start listener on port {}", conf.portin));
    println!("Started listener on port {}", conf.portin);
    for stream in listener.incoming() {
        let mut stream = stream.expect("Could not open stream");
        network::handle_stream(&mut stream, &conf);
    }
}
