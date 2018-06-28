#[macro_export]
macro_rules! log {
    ($logger:expr, $msg:expr, $level:expr, $data:expr) => {{
        {
            let src = $crate::__Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::__Record {
                data: Some($data),
                level: $level,
                msg: $msg,
                src: src,
            };
            $logger.log(record)
        }
    }};
    ($logger:expr, $msg:expr, $level:expr) => {{
        {
            let src = $crate::__Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::__Record {
                data: None as Option<()>,
                level: $level,
                msg: $msg,
                src: src,
            };
            $logger.log(record)
        }
    }};
}

#[macro_export]
macro_rules! json_logger_log {
    ($logger:expr, $msg:expr, $level:expr, $data:expr) => {{
        {
            let src = $crate::__Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::__Record {
                data: Some($data),
                level: $level,
                msg: $msg,
                src: src,
            };
            $logger.log(record)
        }
    }};
    ($logger:expr, $msg:expr, $level:expr) => {{
        {
            let src = $crate::__Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::__Record {
                data: None as Option<()>,
                level: $level,
                msg: $msg,
                src: src,
            };
            $logger.log(record)
        }
    }};
}

#[macro_export]
macro_rules! fatal {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 60, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 60)
    }};
}

#[macro_export]
macro_rules! json_logger_fatal {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 60, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 60)
    }};
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 50, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 50)
    }};
}

#[macro_export]
macro_rules! json_logger_error {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 50, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 50)
    }};
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 40, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 40)
    }};
}

#[macro_export]
macro_rules! json_logger_warn {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 40, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 40)
    }};
}


#[macro_export]
macro_rules! info {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 30, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 30)
    }};
}

#[macro_export]
macro_rules! json_logger_info {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 30, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 30)
    }};
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 20, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 20)
    }};
}

#[macro_export]
macro_rules! json_logger_debug {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 20, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 20)
    }};
}

#[macro_export]
macro_rules! trace {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 10, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 10)
    }};
}

#[macro_export]
macro_rules! json_logger_trace {
    ($logger:expr, $msg:expr, $data:expr) => {{
        log!($logger, $msg, 10, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        log!($logger, $msg, 10)
    }};
}
