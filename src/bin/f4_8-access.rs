use std::{env, ffi::CString};

use libc::{O_RDONLY, R_OK};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: a.out <pathname>");
        std::process::exit(1);
    }
    let arg = args.get(1).expect("argument error");
    let args_c = CString::new(arg.clone()).unwrap();
    unsafe {
        if libc::access(args_c.as_ptr(), R_OK) < 0 {
            eprintln!("access error for {}", arg);
            std::process::exit(2);
        } else {
            println!("read access OK");
        }
        if libc::open(args_c.as_ptr(), O_RDONLY) < 0 {
            eprintln!("open error for {}", arg);
            std::process::exit(3);
        } else {
            println!("open for reading OK");
        }
    }
    std::process::exit(0);
}
