//! JSON Logger
//!
//! This logger follows the [Bunyan](https://github.com/trentm/node-bunyan) logging format.
//!
//! ### Example
//!
//! ```rust
//! #[macro_use]
//! extern crate json_logger;
//! #[macro_use]
//! extern crate serde_json;
//!
//! use json_logger::Logger;
//!
//! fn main() {
//!     let mut log = Logger::new("json_logger", ::std::io::stdout());
//!     json_info!(log, "baz", json!({"a":1, "b":2}));
//!     json_info!(log, "bar", json!({"a":3, "b":4}));
//! }
//! ```

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
#[cfg(windows)]
extern crate winapi;

use serde::Serialize;
use std::io::Write;
use std::process;

mod hostname;
#[macro_use]
mod macros;

const NEWLINE: u8 = 10;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Level {
    Trace = 10,
    Debug = 20,
    Info = 30,
    Warn = 40,
    Error = 50,
    Fatal = 60,
    Disabled = 255,
}

impl From<Level> for u8 {
    fn from(level: Level) -> Self {
        level as u8
    }
}

#[derive(Debug)]
pub struct Logger<W: Write> {
    writer: W,
    level: u8,
    name: String,
    hostname: String,
    pid: u32,
}

impl<W: Write> Logger<W> {
    pub fn new<S: Into<String>>(name: S, writer: W) -> Self {
        Logger {
            writer,
            level: 30,
            name: name.into(),
            hostname: hostname::hostname(),
            pid: process::id(),
        }
    }
    pub fn set_minimum_level<T: Into<u8>>(&mut self, level: T) {
        self.level = level.into()
    }
    pub fn is_enabled<T: Into<u8>>(&self, level: T) -> bool {
        self.level <= level.into()
    }
    pub fn log<T: Serialize>(&mut self, record: Record<T>) {
        if !self.is_enabled(record.level) {
            return
        }
        let serializable_record = SerializableRecord {
            data: record.data,
            name: self.name.as_str(),
            msg: record.msg,
            level: record.level,
            hostname: self.hostname.as_str(),
            pid: self.pid,
            src: record.src,
            time: &time::now_utc().rfc3339().to_string(),
            v: 0,
        };
        if let Ok(mut bytes) = serde_json::to_vec(&serializable_record) {
            bytes.push(NEWLINE);
            let _ = self.writer.write_all(&bytes);
        };
    }
}

pub struct Record<'a, T: Serialize> {
    pub data: Option<T>,
    pub level: u8,
    pub msg: &'a str,
    pub src: Src<'a>,
}

#[derive(Serialize)]
pub struct Src<'a> {
    pub module_path: &'a str,
    pub file: &'a str,
    pub line: u32,
}

#[derive(Serialize)]
struct SerializableRecord<'a, T: Serialize> {
    #[serde(skip_serializing_if="Option::is_none")]
    data: Option<T>,
    level: u8,
    hostname: &'a str,
    msg: &'a str,
    name: &'a str,
    pid: u32,
    src: Src<'a>,
    time: &'a str,
    v: u8
}