#![allow(dead_code)]

use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::io::{self};

const WAIT_TIME: Duration = Duration::from_millis(10);
const REQUEST_CAPACITY: u32 = 10;
const FINISHED: bool = true;
const IP: &str = "127.0.0.1";
const PORT: &str = "12000";

fn main() -> io::Result<()> {

    let listener = TcpListener::bind(&format!("{}:{}", IP, PORT))?;
    let (tx, rx): (Sender<TcpStream>, Receiver<TcpStream>) = channel();
    
    let mut request_handler = RequestHandler::new(rx);
    let handle = thread::spawn(move || {
        request_handler.run();
    });

    loop {
        match listener.accept() {
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

struct Request {
    connection: TcpStream,
    tx: Sender<bool>,
}

impl Request {

    fn run(&self) {
        println!("Request started! {:?}", thread::current());
        self.tx.send(FINISHED);
    }

}

struct RequestHandler {
    waiting_requests: Queue<Request>,
    running_tasks: u32,
    rx_task_callback: Receiver<bool>,
    tx_for_tasks: Sender<bool>,
    rx_new_request:  Receiver<TcpStream>,
}

impl RequestHandler {

    pub fn new(rx_new_request: Receiver<TcpStream>) -> Self {
        let (tx, rx) = channel();
        
        RequestHandler {
            waiting_requests: Queue::new(),
            running_tasks: 0,
            rx_task_callback: rx,
            tx_for_tasks: tx,
            rx_new_request,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.rx_new_request.recv_timeout(WAIT_TIME) {
                Ok(new_request) => {
                    self.add_request(new_request);
                },
                Err(_e) => {},     // Timeout reached
            };
            
            match self.rx_task_callback.recv_timeout(WAIT_TIME) {
                Ok(_) => {
                    // If there's any requests waiting in the buffer, execute them
                    self.running_tasks -= 1;
                    while self.running_tasks < REQUEST_CAPACITY && 
                                 !self.waiting_requests.is_empty() {
                        let request = self.waiting_requests.take();
                        self.execute(request);
                    }
                },
                Err(_e) => {},      // Timeout reached
            };
        }
    }

    fn execute(&mut self, request: Request) {
        thread::spawn(move || {
            request.run();
        });
        self.running_tasks += 1;
    }

    fn add_request(&mut self, stream: TcpStream) {

        let new_request = Request {
            connection: stream,
            tx: self.tx_for_tasks.clone(),
        };

        if self.running_tasks < REQUEST_CAPACITY {
            self.execute(new_request);
        } else {
            self.waiting_requests.put(new_request);
        }
    }

}

struct Queue<T> {
    list: Vec<T>
}

impl<T> Queue<T> {

    pub fn new() -> Self {
        Queue {
            list: Vec::new(),
        }
    }
    
    pub fn put(&mut self, obj: T) {
        self.list.push(obj);
    }

    pub fn take(&mut self) -> T {
        self.list.remove(0)
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    pub fn is_ready(&self) -> bool {
        self.list.len() > 0
    }

}