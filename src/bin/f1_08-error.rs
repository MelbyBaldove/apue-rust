use std::{env, ffi::CString};

use errno::{set_errno, Errno};

fn main() {
    unsafe {
        // strerror maps errnum to an error message
        let eacces_err = CString::from_raw(libc::strerror(libc::EACCES));
        eprintln!("EACCES: {}", eacces_err.to_str().unwrap());
        set_errno(Errno(libc::ENOENT));
        let arg_c = CString::new(env::args().nth(0).unwrap()).unwrap();
        // perror interpolates the argument with the error message
        // in the following format: "<arg>: <error>"
        libc::perror(arg_c.as_ptr());
        std::process::exit(0);
    }
}
