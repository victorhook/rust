extern crate chrono;
use chrono::{DateTime, Utc};

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