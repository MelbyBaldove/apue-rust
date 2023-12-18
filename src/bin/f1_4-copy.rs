use libc::{STDIN_FILENO, STDOUT_FILENO};
const BUFFSIZE: usize = 4096;

fn main() {
    let mut n;
    let mut buf: [u8; BUFFSIZE] = [0; BUFFSIZE];
    let buf_ptr = buf.as_mut_ptr() as *mut libc::c_void;
    unsafe {
        loop {
            n = libc::read(STDIN_FILENO, buf_ptr, BUFFSIZE);
            if n <= 0 {
                break;
            }

            if libc::write(STDOUT_FILENO, buf_ptr, n as usize) != n {
                panic!("write error");
            }
        }

        if n < 0 {
            eprintln!("read error");
        }
        std::process::exit(0);
    }
}
