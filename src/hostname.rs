use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[cfg(unix)]
extern "C" {
    fn gethostname(name: *mut c_char, size: usize) -> c_int;
}

#[cfg(windows)]
unsafe fn gethostname(name: *mut c_char, size: usize) -> c_int {
    use winapi::um::winsock2;

    let mut wsa_data: winsock2::WSADATA = ::std::mem::uninitialized();
    let startup_error_code = winsock2::WSAStartup(514, &mut wsa_data);
    if startup_error_code != 0 {
        return startup_error_code;
    }
    let hostname_error_code = winsock2::gethostname(name as *mut i8, size as c_int);
    winsock2::WSACleanup();
    hostname_error_code
}

pub fn hostname() -> String {
    let mut buf = [0u8; 256];
    let ptr = buf.as_mut_ptr() as *mut c_char;
    unsafe {
        if gethostname(ptr, buf.len()) != 0 {
            String::default()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}
