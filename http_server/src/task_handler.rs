use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Result, BufReader, BufWriter, BufRead};


pub struct Request {
    stream: TcpStream,
    address: SocketAddr,
}

impl Request {
   
    pub fn new(stream: TcpStream, address: SocketAddr) -> Self {
        Request {stream, address}
    }

    pub fn run(stream: TcpStream) -> Result<()> {
        
        let http_request = Request::read_request(stream)?;
        println!("{}", http_request);

        Ok(())
    }

    fn read_request(stream: TcpStream) -> Result<(String)> {
        let mut buff = String::new();
        let mut request = String::new();
        let mut reader = BufReader::new(stream);

        while !buff.eq(EOF) {
            reader.read_line(&mut buff)?;
            request.push_str(&buff);
            buff.clear();
        }
        Ok(request)
    }

    fn send_file() -> Result<()> {

        Ok(())
    }

    fn read_data() -> Result<()> {

        Ok(())
    }

}

use std::thread;

const EOF: &str = "\r\n";

pub struct RequestHandler<T> {
    requests: Vec<T>,
    capacity: u32,
}

impl<T> RequestHandler<T> {
    pub fn new(capacity: u32) -> Self {
        RequestHandler {
            requests: Vec::new(),
            capacity,
        }
    }

    pub fn run(&self) -> Result<()> {
        
        
        Ok(())
    }
}

impl<T> Queue<T> for RequestHandler<T> {

    fn add(&mut self, obj: T) -> bool {
        if self.capacity < self.requests.len() as u32 {
            self.requests.push(obj);
            return true;
        }
        false
    }

    fn remove(&mut self) -> Option<T> {
        if self.requests.len() > 0 {
            return Some(self.requests.remove(0));
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.requests.len() > 0
    }

    fn is_full(&self) -> bool {
        self.requests.len() as u32 == self.capacity
    }

}


pub trait Queue<T> {

    fn add(&mut self, obj: T) -> bool;

    fn remove(&mut self) -> Option<T>;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;

}


