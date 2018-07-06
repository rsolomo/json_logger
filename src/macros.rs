#[macro_export]
macro_rules! json_log {
    ($logger:expr, $msg:expr, $level:expr, $data:expr) => {{
        {
            let src = $crate::Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::Record {
                data: Some($data),
                level: $level,
                msg: $msg.as_ref(),
                src: src,
            };
            $logger.log(record)
        }
    }};
    ($logger:expr, $msg:expr, $level:expr) => {{
        {
            let src = $crate::Src {
                file: file!(),
                line: line!(),
                module_path: module_path!(),
            };
            let record = $crate::Record {
                data: None as Option<()>,
                level: $level,
                msg: $msg.as_ref(),
                src: src,
            };
            $logger.log(record)
        }
    }};
}

#[macro_export]
macro_rules! json_fatal {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 60, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 60)
    }};
}

#[macro_export]
macro_rules! json_error {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 50, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 50)
    }};
}

#[macro_export]
macro_rules! json_warn {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 40, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 40)
    }};
}

#[macro_export]
macro_rules! json_info {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 30, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 30)
    }};
}

#[macro_export]
macro_rules! json_debug {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 20, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 20)
    }};
}

#[macro_export]
macro_rules! json_trace {
    ($logger:expr, $msg:expr, $data:expr) => {{
        json_log!($logger, $msg, 10, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        json_log!($logger, $msg, 10)
    }};
}
