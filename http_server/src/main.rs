use std::io;
use std::net::{TcpStream, TcpListener};
use std::str;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

mod request_handler;
mod http_request;
mod log;

use log::{Logger, LogLevel};
use request_handler::RequestHandler;

const WAIT_TIME: Duration = Duration::from_millis(10);
const REQUEST_CAPACITY: u32 = 10;
const IP: &str = "127.0.0.1";
const PORT: &str = "12000";
const LOG_BASE: &str = "log/";

fn main() -> io::Result<()> {

    // Start up the server, listening to given port
    let listener = TcpListener::bind(&format!("{}:{}", IP, PORT))?;

    // Create a channel-pair to be used for communication between the maind
    // server thread and the request-handler thread. 
    let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = channel();
    
    // Set up logger with given name and level
    let logger = Logger::new(&format!("{}development", LOG_BASE), LogLevel::DEBUG);

    // "request_handler" is responsible for handling the incoming HTTP-requests
    // which is passed from the "listener". It is responsible for answering the requests
    // and to buffer them if the total number exceeds the CAPACITY. This is to ensure
    // that the server isn't overloaded.
    let mut request_handler = RequestHandler::new(rx, logger);
    request_handler.set_request_capacity(REQUEST_CAPACITY);

    // The "request_handler" alternates between listetning to
    // new requests from main thread, and finished request-threads.
    request_handler.set_poll_duration(WAIT_TIME);           

    // Start the new thread for the "request_handler"
    let handle = thread::spawn(move || {
        request_handler.run();
    });

    // Enter infinite listening mode
    loop {
        match listener.accept() {
            // Upon new TCP Request, the stream is sent through the channel
            // to the "request_handler" which takes over from that point.
            Ok((stream, address)) => match tx.send(stream) {
                Ok(_) => {},
                Err(_e) => {},
            },
            Err(_e) => {},
        }
    }
    
    handle.join();
    Ok(())
}