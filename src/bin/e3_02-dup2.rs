use std::ffi::CString;

use libc::{O_RDONLY, SEEK_CUR};

// Implement dup2() without using fcntl()
fn dup2(fd: i32, fd2: i32) -> i32 {
    if fd == fd2 {
        return fd2;
    }
    let mut temp_fds = Vec::new();
    unsafe {
        // check if original fd is valid
        if libc::lseek(fd, 0, SEEK_CUR) == -1 {
            return -1;
        }
        libc::close(fd2);
        loop {
            let temp_fd = libc::dup(fd);
            if temp_fd == -1 {
                return -1;
            }

            if temp_fd >= fd2 {
                for fd in temp_fds {
                    libc::close(fd);
                }
                return temp_fd;
            }

            temp_fds.push(temp_fd);
        }
    }
}

fn main() {
    unsafe {
        let stdin = CString::new("/dev/fd/0").unwrap();
        let fd = libc::open(stdin.as_ptr(), O_RDONLY);
        println!("stdin fd: {}", fd);
        let dup_fd = dup2(fd, 1337);
        println!("dup fd: {}", dup_fd);
    }
}
