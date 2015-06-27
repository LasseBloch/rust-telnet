#![crate_name = "telnet_server"]
#![crate_type = "lib"]

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;

pub struct TelnetServer;

impl TelnetServer {

    fn handle_client(mut stream: TcpStream) {
        println!("Connected");
        println!("peer socketaddr {}", stream.peer_addr().unwrap());
        println!("local socketaddr {}", stream.local_addr().unwrap());
        let mut request = String::new();
        stream.read_to_string(&mut request)
        .ok()
        .expect("failed to read line");
        println!("Read from socket: {}", request);
    }

    pub fn start_server(&self, port: u16) {
        // accept connections and process them, spawning a new thread for each
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        // connection succeede
                        TelnetServer::handle_client(stream);
                    });
                }
            Err(e) => { println!("Error: {:?}", e) }
            }
        }
        // close the socket server
        drop(listener);
    }
}
