#![allow(dead_code)]

use std::net::{TcpStream, TcpListener, Shutdown};
use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::io::{self, Write, Read, BufWriter, BufReader, BufRead};
use std::str;
use std::fs;
use std::path::Path;

const WAIT_TIME: Duration = Duration::from_millis(10);
const REQUEST_CAPACITY: u32 = 10;
const FINISHED: bool = true;
const IP: &str = "127.0.0.1";
const PORT: &str = "12000";

mod HttpResponse;
mod log;

use HttpResponse::Response;

/* HTTP response codes */
const STATUS_OK: &str = "HTTP/1.1 200 OK\r\n";
const STATUS_NOT_MOD: &str = "HTTP/1.1 304 Not modified\r\n";
const STATUS_BAD_REQ: &str = "HTTP/1.1 400 Bad request\r\n";
const STATUS_UN_AUTH: &str = "HTTP/1.1 401 Unauthorized\r\n";
const STATUS_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden\r\n";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not found\r\n";
const STATUS_METH_NOT_ALLOWED: &str = "HTTP/1.1 405 Method not allowed\r\n";

// Content base
const CONTENT_HTML: &str = "text/html; charset=UTF-8";
const CONTENT_CSS: &str = "text/css";
const CONTENT_JS: &str = "text/javascript";
const CONTENT_JPEG: &str = "image/jpeg";
const CONTENT_PNG: &str = "image/png";
const CONTENT_GIF: &str = "image/gif";
const CONTENT_ICO: &str = "image/x-icon";

// Default paths
const REQUEST_BASE: &str = "frontend";
const DEFAULT_FILE: &str = "frontend/html/index.html";
const FILE_404: &str = "frontend/html/404.html";


const END_OF_MSG: &str = "\r\n";
const OVER: &[u8] = "\r\n".as_bytes();


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

        let mut reader = BufReader::new(&self.connection);
        let mut writer = BufWriter::new(&self.connection);

        let mut line = String::new();
        let mut request = String::new();

        while !line.eq(END_OF_MSG) {
            line.clear();
            reader.read_line(&mut line);
            request.push_str(&line);
        }
        //println!("{}", request);

        let req: Vec<&str> = request.split("\n").collect();
        
        let first_line: Vec<&str> = req[0].split(" ").collect();
        let method = first_line[0];
        
        let mut file = match first_line[1].eq_ignore_ascii_case("/") {
            true => "index.html",
            false => first_line[1],
        };

        let mut keep_alive = false;

        match method {
            "GET" => {
                    // TODO: Check for special header responses, Keep-Alive etc
                    for line in 1..req.len() {
                        let header = req[line].split(" ").next().unwrap();
                        if header.eq("Connection:") {
                            keep_alive = req[line + 1].split(" ").next().unwrap().eq("Keep-alive");
                        }
                    }

                    // Checks the file extension to build the right path, and then
                    // ensure that the file exists before proceeding.
                    // If the requested file can't be found, a default one i served.
                    // (usually index.html)
                    println!("\n    New Request ");
                    println!("[*] Before extension check: {}", file);
                    let (file, status_code, content_type) = self.get_path(file);
                    println!("[*] After extension check: {}", file);

                    let file = match Path::new(&file).exists() {
                        true => &file,
                        false => DEFAULT_FILE,
                    };

                    println!("[*] After path exists check: {}", file);

                    let (data, status_code, content_type) = match fs::read(file) {
                        Ok(content) => (content, status_code, content_type),
                        Err(_e) => {
                            println!("Yes!");
                            return ();
                        },
                    };

                    let mut response = Response::new(status_code, content_type, data.len());
                    writer.write_all(response.unpack());
                    writer.flush();
                    writer.write_all(&data[..]);
                    writer.flush();
                    writer.write(OVER);
                    writer.flush();


                    if keep_alive {
                        // Todo #FIX
                    }

            },
            _ => {},
        }

        drop(self);
    }

    fn get_path(&self, file: &str) -> (String, &str, &str) {
        
        let filename = match file.contains("/") {
            true => String::from(file),
            false => String::from(format!("/{}", file)),
        };

        match file.split(".").last() {
            Some(ext) => {
                match ext {
                    "html" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_HTML),
                    "css" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_CSS),
                    "js" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_JS),
                    "jpg" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_JPEG),
                    "png" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_PNG),
                    "gif" => (format!("{}{}", REQUEST_BASE, filename), STATUS_OK, CONTENT_GIF),
                    _ => (String::from(FILE_404), STATUS_NOT_FOUND, CONTENT_HTML),
                }
            },
            // Filename doens't contain a period .
            None => {
                println!("NOT FOUND!");
                return (String::from(DEFAULT_FILE), STATUS_OK, CONTENT_HTML);
            },
        }
    }

    fn file_exist(&self, file: &str) -> bool {
        Path::new(&format!("{}{}", REQUEST_BASE, file)).exists()
    }
 
    fn new(connection: TcpStream, tx: Sender<bool>) -> Self {
        Request {
            connection,
            tx,
        }
    }

}

impl Drop for Request {
    fn drop(&mut self) {
        self.connection.shutdown(Shutdown::Both);
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
        let new_request = Request::new(stream, self.tx_for_tasks.clone());

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