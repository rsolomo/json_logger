#[macro_use]
extern crate json_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use json_logger::{Logger, Level};
use serde_json::Value;
use std::str;

#[derive(Debug, Deserialize)]
struct Record<'a> {
    data: Option<Value>,
    hostname: &'a str,
    level: i64,
    msg: &'a str,
    name: &'a str,
    pid: i64,
    src: Value,
    time: &'a str,
}

#[test]
fn log() {
    let mut buffer: Vec<u8> = vec![];
    {
        let mut log = Logger::new("json_logger", &mut buffer);
        json_log!(log, "baz", 30);
    }
    let actual: Record = serde_json::from_slice(&buffer).unwrap();

    assert!(!actual.data.is_some());
    assert!(!actual.hostname.is_empty());
    assert_eq!(actual.level, 30);
    assert_eq!(actual.msg, "baz");
    assert!(!actual.time.is_empty());
}

#[test]
fn log_with_data() {
    #[derive(Serialize, Debug)]
    struct Data<'a> {
        a: &'a str,
        b: i64
    }

    let mut buffer: Vec<u8> = vec![];
    {
        let mut log = Logger::new("json_logger", &mut buffer);
        json_log!(log, "baz", 40, Data {a: "1", b: 2});
    }
    let value: Value = serde_json::from_slice(&buffer).unwrap();
    let actual = value.pointer("/data").unwrap().as_object().unwrap();

    assert_eq!(actual["a"].as_str(), Some("1"));
    assert_eq!(actual["b"].as_i64(), Some(2));
}

#[test]
fn log_level_macros() {
    let mut buffer: Vec<u8> = vec![];
    {
        let mut log = Logger::new("json_logger", &mut buffer);
        log.set_minimum_level(Level::Trace);
        json_fatal!(log, "test");
        json_error!(log, "test");
        json_warn!(log, "test");
        json_info!(log, "test");
        json_debug!(log, "test");
        json_trace!(log, "test");
        json_log!(log, "test", 30);
    }
    let s = str::from_utf8(&buffer).unwrap();
    let json_iterator = s.split_terminator('\n');
    let logs: Vec<Record> = json_iterator.map(serde_json::from_str).map(Result::unwrap).collect();

    assert_eq!(logs[0].level, 60);
    assert_eq!(logs[1].level, 50);
    assert_eq!(logs[2].level, 40);
    assert_eq!(logs[3].level, 30);
    assert_eq!(logs[4].level, 20);
    assert_eq!(logs[5].level, 10);
    assert_eq!(logs[6].level, 30);
}

#[test]
fn set_minimum_level() {
    let mut buffer: Vec<u8> = vec![];
    {
        let mut log = Logger::new("json_logger", &mut buffer);
        log.set_minimum_level(Level::Info);
        json_info!(log, "visible");
        json_trace!(log, "not visible");
        log.set_minimum_level(Level::Error);
        json_info!(log, "not visible");
        json_fatal!(log, "visible");
        log.set_minimum_level(Level::Disabled);
        json_fatal!(log, "not visible");
    }
    let s = str::from_utf8(&buffer).unwrap();
    let json_iterator = s.split_terminator('\n');
    let logs: Vec<Record> = json_iterator.map(serde_json::from_str).map(Result::unwrap).collect();

    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].msg, "visible");
    assert_eq!(logs[1].msg, "visible");
}
