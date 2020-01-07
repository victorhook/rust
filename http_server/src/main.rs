#![allow(dead_code)]

use std::net::{TcpListener, TcpStream};
use std::io::{Result};
use std::thread;

mod task_handler;
use task_handler::{RequestHandler, Request, Queue};

const IP: &str = "127.0.0.1";
const PORT: &str = "12000";
const REQUEST_CAPACITY: u32 = 1000;

mod test;

fn main() -> Result<()> {
    
    let listener = TcpListener::bind(format!("{}:{}", IP, PORT))?;
    let mut task_handler: RequestHandler<Request> = RequestHandler::new(REQUEST_CAPACITY);
    
    loop {

        match listener.accept() {
            Ok((stream, address)) => {
                if !task_handler.is_empty() {
                    task_handler.add(Request::new(stream, address));
                } else {
                    // Todo: Handle full task_handler
                }
            }
            // Todo: LOGGER
            Err(_e) => return Err(_e),
        }



    }

    Ok(())
}

