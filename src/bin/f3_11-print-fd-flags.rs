use std::env;

use libc::{F_GETFL, O_ACCMODE, O_APPEND, O_NONBLOCK, O_RDONLY, O_RDWR, O_SYNC, O_WRONLY};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: a.out <descriptor#>");
        std::process::exit(1);
    }
    unsafe {
        let fd = args[1].parse().unwrap();
        let val = libc::fcntl(fd, F_GETFL, 0);
        if val < 0 {
            eprintln!("fcntl error for fd {}", fd);
        }
        match val & O_ACCMODE {
            O_RDONLY => {
                print!("read only");
            }
            O_WRONLY => {
                print!("write only");
            }
            O_RDWR => {
                print!("read write");
            }
            _ => {
                eprintln!("unknown access mode");
            }
        }
        if val & O_APPEND != 0 {
            print!(", append");
        }
        if val & O_NONBLOCK != 0 {
            print!(", nonblocking");
        }
        if val & O_SYNC != 0 {
            print!(", synchronous writes");
        }
        print!("\n");
        std::process::exit(0);
    }
}
