use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{self, BufReader, BufRead, Read, Write, Result};
use std::fs::{File, metadata};
use std::error::Error;

extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

const ADDR: &str = "127.0.0.1";
const PORT: i32 = 8080;
const EOF: &str = "\r\n";
const UNKOWN_METHOD: &str = "unknown";
const UNKOWN_HOST: &str = "unknown";


use std::thread;



fn main() -> Result<()> {
    


    
    // Creates a TCPListenter on the given port and returns an error
    // if the port can't be used (eg: port 80 is often forbidden by the OS).
    let listener = TcpListener::bind(format!("{}:{}", ADDR, PORT))?;
    let (stream, addr) = listener.accept()?;

    
    let t = thread::spawn( move || RequestHandler::run(stream, addr) );
    
    t.join();

    /*
    let mut file = File::create("www.google.com/index.html");
    match file {
        Ok(mut file) => {
            match file.write("test".as_bytes()) {
                Ok(nice) => println!("Wrote to file! {}", nice),
                Err(_e) => println!("Error! {:?}", _e),
            }
        },
        Err(_e) => println!("Error! {:?}", _e),
    }
    */
    /*
    //save_data("test".to_string(), "testfile");
    match send_get("www.google.com", 80, &msg, "testfile") {
        Ok(_) => {},
        Err(e) => println!("{}", e.description()),
    }
    */

    Ok(())

}

struct RequestHandler;

impl RequestHandler {
    fn run(stream: TcpStream, addr: SocketAddr) -> Result<()> {
        println!("Connected to with client {:?}", addr.ip());

        let mut reader = BufReader::new(&stream);
        let mut request = String::new();
        let mut line = String::new();
    
        while !line.eq(EOF) {
            line.clear();
            reader.read_line(&mut line)?;
            request.push_str(&line);
        }

        let method = match request.split(" ").next() {
            Some(method_str) => method_str,
            None => UNKOWN_METHOD,
        };

        let host = match request.split(" ").nth(1) {
            Some(host_name) => &host_name[1..],
            None => UNKOWN_HOST,
        };

        let file_name = match request.split("/").nth(2) {
            Some(file) => if !file.eq("") {"/index.html".to_string()} else {format!("/{}", file)},
            None => "/index.html".to_string(),
        };
        
        if method.eq("GET") {
            let success = match RequestHandler::send_get(&host, &file_name) {
                Ok(data) => RequestHandler::send_response(stream, data),
                Err(_e) => Err(_e),
            };
            return success;
        }

        Ok(())
    }

    fn send_response(mut stream: TcpStream, data: String) -> Result<()> {
        println!("Sending response");
        stream.write(data.as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn send_get(host: &str, file_name: &str) -> Result<(String)> {
    
        let mut stream = TcpStream::connect(format!("{}:{}", host, 80))?;
    
        // Header needed for deciding wether we need a new page or not
        let metadata = metadata(file_name)?.modified()?;
        let last_mod: DateTime<Utc> = metadata.into();

        stream.write(format!("GET {}{} HTTP/1.1\n", host, file_name).as_bytes())?;
        stream.write("Host: localhost\n".as_bytes())?;
        stream.write("User-Agent: Rust Proxy\n".as_bytes())?;
        stream.write("Accept: text/html,application/xhtml+xml\n".as_bytes())?;
        stream.write(format!("If-Modified-Since: {}\n", 
                                    last_mod.format("%d/%m/%Y %T\n")).as_bytes())?;
        stream.write("\n\n".as_bytes())?;
        stream.flush()?;
       
        // Reads the response from the request
        let mut reader = BufReader::new(stream);
        let mut data = String::new();
        let mut line = String::new();
    
        while !line.eq(EOF) {
            line.clear();
            reader.read_line(&mut line)?;
            data.push_str(&line);
        }
        println!("Sent request");

        // If no error occurs, the newly recieved data is saved to the cache
        // and the data is returned 
        match RequestHandler::save_data(&mut data, &format!("{}{}", host, file_name)) {
            Ok(_) => { println!("=>{}", data); return Ok(data); },
            Err(_e) => { println!("{:?}", _e); return Err(_e); },
        }
    }

    /* Saves the data to the given file name, and returns the result */
    fn save_data(data: &mut String, file_name: &str) -> Result<()> {
        println!("{}", file_name);
        let mut file = File::create(file_name)?;        // Returns error if an error occurs
        file.write_all(data.as_bytes())?;               // Returns error if an error occurs 
        Ok(())                                          // If no error occurs, an Ok is returned
    }

}





