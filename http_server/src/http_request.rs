/* External crates */
extern crate chrono;
use chrono::Utc;

use std::fs;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::net::{TcpStream, Shutdown};
use std::sync::mpsc::{Sender};
use std::path::Path;

/* Message short-commands */
const END_OF_MSG: &str = "\r\n";
const OVER: &[u8] = "\r\n".as_bytes();

/* HTTP response codes */
const STATUS_OK: &str = "HTTP/1.1 200 OK\r\n";
const STATUS_NOT_MOD: &str = "HTTP/1.1 304 Not modified\r\n";
const STATUS_BAD_REQ: &str = "HTTP/1.1 400 Bad request\r\n";
const STATUS_UN_AUTH: &str = "HTTP/1.1 401 Unauthorized\r\n";
const STATUS_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden\r\n";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not found\r\n";
const STATUS_METH_NOT_ALLOWED: &str = "HTTP/1.1 405 Method not allowed\r\n";

/* Content base */
const CONTENT_HTML: &str = "text/html; charset=UTF-8";
const CONTENT_CSS: &str = "text/css";
const CONTENT_JS: &str = "text/javascript";
const CONTENT_JPEG: &str = "image/jpeg";
const CONTENT_PNG: &str = "image/png";
const CONTENT_GIF: &str = "image/gif";
const CONTENT_ICO: &str = "image/x-icon";

/* Default paths */
const REQUEST_BASE: &str = "frontend";
const DEFAULT_FILE: &str = "frontend/html/index.html";
const FILE_404: &str = "frontend/html/404.html";

/* Misc */
const FINISHED: bool = true;

pub struct Response {
    response: String,
    extra_headers: Vec<String>,
}

impl Response {

    pub fn new(status_line: &str, content_type: &str, content_length: usize) -> Self {
        let date = Utc::now().format("%a, %d %h %Y %H:%M:%S");

        let mut response = String::new();
        response.push_str(status_line);
        response.push_str(&format!("Date: {}\r\n", date));
        response.push_str(&format!("Content-type: {}\r\n", content_type));
        response.push_str(&format!("Conent-length: {}\r\n", content_length));
        response.push_str("Server: Rust Http-Server\r\n");

        Response {
            response,
            extra_headers: Vec::new(),
        }
    }

    pub fn add_header(&mut self, header: String) {
        self.response.push_str(&header);
        self.response.push_str("\r\n");
    }

    pub fn unpack(&mut self) -> &[u8] {
        self.response.push_str("\r\n");
        self.response.as_bytes()
    }

}


pub struct Request {
    connection: TcpStream,
    tx: Sender<bool>,
}


impl Request {

    pub fn run(&self) {

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

    pub fn get_addr(&self) -> String {
        format!("{}", self.connection.peer_addr().unwrap())
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
 
    pub fn new(connection: TcpStream, tx: Sender<bool>) -> Self {
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