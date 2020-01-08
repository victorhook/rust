use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path};

extern crate chrono;
use chrono::{DateTime, Utc};

const DATE_FORMAT: &str = "%d/%m/%y %H:%M:%S";

pub enum LogLevel {
    INFO,
    WARNING,
    DEBUG,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LogLevel::DEBUG => write!(f, "DEBUG"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::INFO => write!(f, "INFO"),
        }
    }
}

pub struct Logger {
    root: String,
    level: LogLevel,
}

fn main() {

    let logger = Logger::new("log", LogLevel::DEBUG);

}

impl Logger {

    pub fn new(root: &str, level: LogLevel) -> Self {

        let logger = Logger {
            root: String::from(root),
            level,
        };

        match logger.level {
            // On INFO level, the logging is appended to the file for each session
            LogLevel::INFO => {
                let file = match OpenOptions::new().write(true)
                .append(true)
                .open(root) {
                    Ok(file) => file,
                    Err(_e) => File::create(root).unwrap()
                };

            let mut writer = BufWriter::new(file);
            let date = format!("{}\n\n",Utc::now().format(DATE_FORMAT));
            writer.write(format!("New session initialized at {} level\n", logger.level).as_bytes());
            writer.write(date.as_bytes());
            writer.flush();

            },
            // On the DEBUG levels, the logging file gets truncated and overwritten each new session
            LogLevel::DEBUG => {
                File::create(root).unwrap();
            },
            _ => {},
        };

        return logger;
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn log(&self, msg: String) {

        let log_msg = match self.level {
            LogLevel::DEBUG => format!("{}\n", msg),
            LogLevel::INFO => format!("[{}] {}\n", Utc::now().format(DATE_FORMAT), msg),
            // Reserved for later use
            LogLevel::WARNING => format!("[{}] > WARNING: 
                                     {}\n", Utc::now().format(DATE_FORMAT), msg),
        };

    let file = match OpenOptions::new().write(true)
                    .append(true).open(&self.root) {
        Ok(file) => file,
        Err(_e) => File::create(&self.root).unwrap()
    };

    let mut writer = BufWriter::new(file);
    writer.write(log_msg.as_bytes());
    writer.flush();
    }
}