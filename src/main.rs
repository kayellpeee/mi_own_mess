extern crate mio;
#[macro_use]
extern crate log;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled (&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    })
}

fn main() {
    let initialized = init();
    if initialized.is_err() {
        panic!("Logger not initialized");
    }

    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let address: std::net::SocketAddr = "127.0.0.1:13265".parse().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server, SERVER).unwrap();
    info!("Registered server - {:?}", server);

    let sock = TcpStream::connect(&address).unwrap();
    event_loop.register(&sock, CLIENT).unwrap();
    info!("Registered client - {:?}", sock);
    let sender = event_loop.channel();
    info!("Created new channel - {:?}", sender);

    struct MyHandler(TcpListener);
    impl Handler for MyHandler {
        type Timeout = ();
        type Message = i32;

        fn readable(&mut self, event_loop: &mut EventLoop<MyHandler>,
                    token: Token, _: ReadHint) {
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

        fn notify(&mut self, event_loop: &mut EventLoop<MyHandler>, msg: i32) {
            info!("Recieved message - {:?}", msg);
            event_loop.shutdown();
        }
    }
    let response = sender.send(132);
    info!("Sent message - {:?}", response);
    event_loop.run(&mut MyHandler(server)).unwrap();
}
