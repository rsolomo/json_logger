#[macro_use]
extern crate json_logger;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use json_logger::Logger;
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
        log!(log, "baz", 30);
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
        log!(log, "baz", 40, Data {a: "1", b: 2});
    }
    let value: Value = serde_json::from_slice(&buffer).unwrap();
    let actual = value.pointer("/data").unwrap().as_object().unwrap();

    assert_eq!(actual["a"].as_str(), Some("1"));
    assert_eq!(actual["b"].as_i64(), Some(2));
}

#[test]
fn log_levels() {
    let mut buffer: Vec<u8> = vec![];
    {
        let mut log = Logger::new("json_logger", &mut buffer);
        fatal!(log, "test");
        json_logger_fatal!(log, "test");
        error!(log, "test");
        json_logger_error!(log, "test");
        warn!(log, "test");
        json_logger_warn!(log, "test");
        info!(log, "test");
        json_logger_info!(log, "test");
        debug!(log, "test");
        json_logger_debug!(log, "test");
        trace!(log, "test");
        json_logger_trace!(log, "test");
    }
    let s = str::from_utf8(&buffer).unwrap();
    let json_iterator = s.split_terminator('\n');
    let logs: Vec<Record> = json_iterator.map(serde_json::from_str).map(Result::unwrap).collect();

    assert_eq!(logs[0].level, 60);
    assert_eq!(logs[1].level, 60);
    assert_eq!(logs[2].level, 50);
    assert_eq!(logs[3].level, 50);
    assert_eq!(logs[4].level, 40);
    assert_eq!(logs[5].level, 40);
    assert_eq!(logs[6].level, 30);
    assert_eq!(logs[7].level, 30);
    assert_eq!(logs[8].level, 20);
    assert_eq!(logs[9].level, 20);
    assert_eq!(logs[10].level, 10);
    assert_eq!(logs[11].level, 10);
}
