//! JSON Logger
//!
//! This logger follows the [Bunyan](https://github.com/trentm/node-bunyan) logging format.
//!
//! ### Example
//!
//! ```rust,ignore
//!
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

const NEWLINE: &[u8; 1] = &[10];

#[derive(Debug)]
pub struct Logger<W: Write> {
    writer: W,
    log_level: u8,
    name: String,
    hostname: String,
    pid: u32,
}

impl<W: Write> Logger<W> {
    pub fn new<S: Into<String>>(name: S, writer: W) -> Self {
        Logger {
            writer,
            log_level: 30,
            name: name.into(),
            hostname: hostname::hostname(),
            pid: process::id(),
        }
    }
    pub fn log<T: Serialize>(&mut self, record: __Record<T>) {
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
        let result = {
            serde_json::to_writer(&mut self.writer, &serializable_record)
        };
        if result.is_ok() {
            let _ = self.writer.write_all(NEWLINE);
        };
    }
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
    src: __Src,
    time: &'a str,
    v: u8
}

pub struct __Record<'a, T: Serialize> {
    pub data: Option<T>,
    pub level: u8,
    pub msg: &'a str,
    pub src: __Src,
}

#[derive(Serialize)]
pub struct __Src {
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
}
