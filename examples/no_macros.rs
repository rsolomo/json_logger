extern crate json_logger;

use std::panic::{self, PanicInfo};
use json_logger::{Logger, Record, Src};

fn main() {
    panic::set_hook(Box::new(|panic_info: &PanicInfo| {
        let mut log = Logger::new("no_macros", ::std::io::stderr());
        let src = match panic_info.location() {
            // The location information from PanicInfo is more
            // interesting than the line number of this closure.
            Some(loc) => Src {
                file: loc.file(),
                line: loc.line(),
                module_path: ""
            },
            None => Src {
                file: file!(),
                line: line!(),
                module_path: ""
            }
        };
        let msg = match panic_info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        let record = Record {
            data: None as Option<()>,
            level: 60,
            msg,
            src
        };
        log.log(record);
    }));

    panic!("bad things are happening");
}
