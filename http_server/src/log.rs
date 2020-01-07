use std::fs::{File};
use std::io::{BufWriter, Write};
use std::path::{Path};

extern crate chrono;
use chrono::{DateTime, Utc};

pub enum LogLevel {
    info,
    warning,
    debug
}

pub struct Logger {
    root: String,
    level: LogLevel,
}

fn main() {

    let logger = Logger::new("log", LogLevel::info);

}

impl Logger {

    pub fn new(root: &str, level: LogLevel) -> Self {
        
        let file = File::open(root).or_else(|f| {
            File::create(root)}
        ).unwrap();

        let mut writer = BufWriter::new(file);
        writer.write("test".as_bytes());
        writer.flush();
        drop(writer);

        Logger {
            root: String::from(root),
            level,
        }

    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

}

/*
impl Log for Logger {

    fn info(&mut self, msg: &str) {
        
        match File::open(self.root) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                let content = match self.level {
                    LogLevel::debug => {
                        let date = Utc::now().format("%d/%m/%y %H:%M:%S");
                    },
                    LogLevel::info => {},
                    LogLevel::warning => {},
                }

            }
            Err(_e) => {},
        }
    }

}

trait Log {

    fn info(&mut self);

    fn warning(&mut self);

    fn debug(&mut self);

}

*/