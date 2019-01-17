//! JSON Logger
//!
//! This logger follows the [Bunyan](https://github.com/trentm/node-bunyan) logging format.
//!
//! ### Example
//!
//! ```rust,ignore
//! #[macro_use] extern crate log;
//! extern crate json_logger;
//! extern crate rustc_serialize;
//!
//! use log::LevelFilter;
//! use rustc_serialize::json;
//!
//! #[derive(RustcEncodable)]
//! struct LogMessage<'a> {
//!     msg: &'a str,
//!     event: &'a str
//! }
//!
//! fn main() {
//!     json_logger::init("app_name", LevelFilter::Info).unwrap();
//!
//!     // This string will show up in the "msg" property
//!     info!("sample message");
//!
//!     // This will extend the log message JSON with additional properties
//!     info!("{}", json::encode(&LogMessage {
//!         msg: "sample message 2", event: "structured log"
//!     }).unwrap());
//! }
//! ```

extern crate libc;
extern crate log;
extern crate rustc_serialize;
extern crate time;

use libc::{c_char, c_int, getpid, size_t};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use rustc_serialize::json::{self, Json, Object, ToJson};
use std::borrow::ToOwned;
use std::default::Default;
use std::io::prelude::*;
use std::io::{self, Stdout};
use std::str;

extern "C" {
    fn gethostname(name: *mut c_char, len: size_t) -> c_int;
}

pub struct JsonLogger {
    out: Stdout,
    level: LevelFilter,
    name: String,
    hostname: String,
    pid: i32,
}

impl Log for JsonLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let file = record.file().unwrap_or("");
        let line = record.line().map_or(-1, |v| v as i32);
        let module_path = record.module_path().unwrap_or("");

        let mut root = Object::new();
        root.insert("hostname".to_owned(), self.hostname.to_json());
        root.insert(
            "level".to_owned(),
            match record.level() {
                Level::Error => Json::U64(50),
                Level::Warn => Json::U64(40),
                Level::Info => Json::U64(30),
                Level::Debug => Json::U64(20),
                Level::Trace => Json::U64(10),
            },
        );
        root.insert("name".to_owned(), self.name.to_json());
        root.insert("pid".to_owned(), self.pid.to_json());
        root.insert("msg".to_owned(), Json::Null);

        let mut src = Object::new();
        src.insert("module_path".to_owned(), module_path.to_json());
        src.insert("file".to_owned(), file.to_json());
        src.insert("line".to_owned(), line.to_json());

        root.insert("src".to_owned(), Json::Object(src));
        root.insert(
            "time".to_owned(),
            Json::String(time::now_utc().rfc3339().to_string()),
        );
        root.insert("v".to_owned(), Json::U64(0));

        let s = record.args().to_string();
        match Json::from_str(&s).ok() {
            Some(j) => {
                if let Json::Object(obj) = j {
                    root.extend(obj);
                }
            }
            None => {
                // If the log message is not JSON,
                // we will fallback to treating it as a normal string.
                root.insert("msg".to_owned(), Json::String(s));
            }
        }

        if let Ok(s) = json::encode(&root) {
            let _ = writeln!(&mut self.out.lock(), "{}", s);
        }
    }

    fn flush(&self) {}
}

pub fn init(name: &str, level: LevelFilter) -> Result<(), SetLoggerError> {
    let mut buf = vec![0; 255];
    let err = unsafe { gethostname(buf.as_mut_ptr() as *mut c_char, buf.len() as size_t) };
    let hostname = if err == 0 {
        let len = buf.iter().position(|byte| *byte == 0).unwrap_or(buf.len());
        str::from_utf8(&buf[..len])
            .ok()
            .unwrap_or_default()
            .to_string()
    } else {
        String::default()
    };

    let pid = unsafe { getpid() };

    let logger = JsonLogger {
        out: io::stdout(),
        level: level,
        name: name.to_owned(),
        hostname: hostname,
        pid: pid,
    };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level);
    Ok(())
}
