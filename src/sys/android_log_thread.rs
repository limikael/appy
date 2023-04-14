use android_log_sys::{__android_log_write};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::os::unix::prelude::{FromRawFd, RawFd};
use std::io::{BufRead, BufReader};

pub fn spawn_android_log_thread() {
    unsafe {
        let mut logpipe: [RawFd; 2] = Default::default();
        libc::pipe(logpipe.as_mut_ptr());
        libc::dup2(logpipe[1], libc::STDOUT_FILENO);
        libc::dup2(logpipe[1], libc::STDERR_FILENO);
        std::thread::spawn(move || {
            let tag = CStr::from_bytes_with_nul(b"sdl-app\0").unwrap();
            let file = File::from_raw_fd(logpipe[0]);
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            loop {
                buffer.clear();
                if let Ok(len) = reader.read_line(&mut buffer) {
                    if len == 0 {
                        break;
                    } else if let Ok(msg) = CString::new(buffer.clone()) {
                       // 3 means DEBUG
                       __android_log_write(3,tag.as_ptr(),msg.as_ptr());
                    }
                }
            }
        });
    }
}
