use std::ffi::CString;

use libc::{c_void, SEEK_SET, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR};

const BUF1: &[u8; 10] = b"abcdefghij";
const BUF2: &[u8; 10] = b"ABCDEFGHIJ";
fn main() {
    let fd: i32;

    unsafe {
        let file_name = CString::new("file.hole").expect("CString::new failed");
        fd = libc::creat(
            file_name.as_ptr() as *const i8,
            S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH,
        );
        if fd < 0 {
            eprintln!("creat error");
        }

        if libc::write(fd, BUF1.as_ptr() as *const c_void, 10) != 10 {
            eprintln!("BUF1 write error");
        }

        if libc::lseek(fd, 16384, SEEK_SET) == -1 {
            eprintln!("lseek error");
        }

        if libc::write(fd, BUF2.as_ptr() as *const c_void, 10) != 10 {
            eprintln!("BUF2 write error");
        }
    }

    std::process::exit(0);
}
