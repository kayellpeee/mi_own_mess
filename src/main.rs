extern crate mio;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

fn main() {

    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let address: mio::IpAddr = "127.0.0.1:13555".parse().unwrap();

    println!("here is address {:?}", address);

    let server = TcpListener::bind(&address).unwrap();
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server, SERVER).unwrap();
    let sock = TcpStream::connect(&address).unwrap();
    event_loop.register(&sock, CLIENT).unwrap();
    struct MyHandler(TcpListener);
    impl Handler for MyHandler {
        type Timeout = ();
        type Message = ();

        fn readable(&mut self, event_loop: &mut EventLoop<MyHandler>,
                    token: Token, _:ReadHint) {
            match token {
                SERVER => {
                    let MyHandler(ref mut server) = *self;
                    let _ = server.accept();
                }
                CLIENT => {
                    event_loop.shutdown();
                }
                _ => panic!("unexpected token bro"),
            }
        }
    }
    event_loop.run(&mut MyHandler(server)).unwrap();
}
