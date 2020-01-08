use crate::http_request::Request;
use crate::log::Logger;

use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;
use std::thread;

/* Default values for initialization */
const DEFAULT_CAPACITY: u32 = 1000;
const DEFAULT_WAIT_TIME: Duration = Duration::from_millis(10);


pub struct RequestHandler {
    waiting_requests: Queue<Request>,
    request_capacity: u32,              
    running_tasks: u32,
    wait_time: Duration,

    rx_task_callback: Receiver<bool>,
    tx_for_tasks: Sender<bool>,
    rx_new_request:  Receiver<TcpStream>,
    logger: Logger,
}

impl RequestHandler {

    /* Creates a new RequestHandler with an recieve-channel for communication between
        the main thread and this own thread                                           */
    pub fn new(rx_new_request: Receiver<TcpStream>, logger: Logger) -> Self {
        let (tx, rx) = channel();
        
        RequestHandler {
            waiting_requests: Queue::new(),
            running_tasks: 0,
            rx_task_callback: rx,
            tx_for_tasks: tx,
            rx_new_request,
            request_capacity: DEFAULT_CAPACITY,
            wait_time: DEFAULT_WAIT_TIME,
            logger
        }
    }

    /* Method that the thread should be initialized with. The RequestHandler
        enters an infinite loop polling between main thread and callbacks from
        finished request-threads.                                        */ 
    pub fn run(&mut self) {
        loop {     
                // If match == Ok, we've recieved a new TCP Request
            match self.rx_new_request.recv_timeout(self.wait_time) {
                Ok(new_request) => {
                    self.add_request(new_request);
                },
                Err(_e) => {},     // Timeout reached
            };
            
                    // Callback from running task means that it's finished 
            match self.rx_task_callback.recv_timeout(self.wait_time) {
                Ok(_) => {
                    // If there's any requests waiting in the buffer, execute them
                    self.running_tasks -= 1;
                    while self.running_tasks < self.request_capacity && 
                                 !self.waiting_requests.is_empty() {
                        let request = self.waiting_requests.take();
                        self.execute(request);
                    }
                },
                Err(_e) => {},      // Timeout reached
            };
        }
    }

    /* Starts a new thread for the given request */
    pub fn execute(&mut self, request: Request) {
        self.logger.log(format!("New Request started, from: {}", request.get_addr()));
        thread::spawn(move || {
            request.run();
        });
        self.running_tasks += 1;
    }

    /* Starts a new thread for the given request if there's capacity for it,
        otherwise it is stored in the request-buffer (queue)            */
    pub fn add_request(&mut self, stream: TcpStream) {
        let new_request = Request::new(stream, self.tx_for_tasks.clone());

        if self.running_tasks < self.request_capacity {
            self.execute(new_request);
        } else {
            self.waiting_requests.put(new_request);
        }
    }

    pub fn set_request_capacity(&mut self, new_capacity: u32) {
        self.request_capacity = new_capacity;
    }

    pub fn set_poll_duration(&mut self, new_wait_time: Duration) {
        self.wait_time = new_wait_time;
    }

}

/* Simple wrapper for a Vec.
TODO: Perhaps implement a linkedlist instead */
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