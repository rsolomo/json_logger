#[macro_export]
macro_rules! jl_log {
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
                msg: $msg,
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
                msg: $msg,
                src: src,
            };
            $logger.log(record)
        }
    }};
}

#[macro_export]
macro_rules! jl_fatal {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 60, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 60)
    }};
}

#[macro_export]
macro_rules! jl_error {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 50, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 50)
    }};
}

#[macro_export]
macro_rules! jl_warn {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 40, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 40)
    }};
}

#[macro_export]
macro_rules! jl_info {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 30, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 30)
    }};
}

#[macro_export]
macro_rules! jl_debug {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 20, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 20)
    }};
}

#[macro_export]
macro_rules! jl_trace {
    ($logger:expr, $msg:expr, $data:expr) => {{
        jl_log!($logger, $msg, 10, $data)
    }};
    ($logger:expr, $msg:expr) => {{
        jl_log!($logger, $msg, 10)
    }};
}
