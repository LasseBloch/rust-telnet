extern crate telnet_server;

use telnet_server::TelnetServer;

fn main() {
    let tel_server = TelnetServer;
    tel_server.start_server(8080);
}
