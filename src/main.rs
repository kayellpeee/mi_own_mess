extern crate mio;
#[macro_use]
extern crate log;

use mio::*;
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

    let mut event_loop = EventLoop::new().unwrap();

    let sender = event_loop.channel();
    info!("Created new channel - {:?}", sender);

    struct MyHandler;
    impl Handler for MyHandler {
        type Timeout = ();
        type Message = i32;

        fn notify(&mut self, event_loop: &mut EventLoop<MyHandler>, msg: i32) {
            info!("Recieved message - {:?}", msg);
            event_loop.shutdown();
        }
    }
    let response = sender.send(132);
    info!("Sent message - {:?}", response);
    event_loop.run(&mut MyHandler).unwrap();
}
