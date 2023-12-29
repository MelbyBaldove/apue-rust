use std::ffi::CString;

use libc::{c_void, O_APPEND, O_RDWR, SEEK_SET};

fn main() {
    let file_path = CString::new("to_replace.txt").unwrap();
    let new_str = CString::new("replaced").unwrap();
    unsafe {
        let fd = libc::open(file_path.as_ptr(), O_RDWR | O_APPEND);
        libc::lseek(fd, 0, SEEK_SET);
        // moving the offset to the start of file has no effect on subsequent write call
        // which still writes to the end of file
        libc::write(
            fd,
            new_str.as_ptr() as *const c_void,
            new_str.to_bytes().len(),
        );
    }

    std::process::exit(0);
}
