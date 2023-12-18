use std::{
    env,
    ffi::{CStr, CString},
};

fn main() {
    let dir = env::args().nth(1).expect("usage: ls directory_name");

    unsafe {
        let c_dir = CString::new(dir).unwrap();
        let dir_p = c_dir.as_ptr();

        let dp = libc::opendir(dir_p);

        if dp.is_null() {
            eprintln!("can't open {:?}", c_dir);
            std::process::exit(1);
        }

        loop {
            let dirp = libc::readdir(dp);
            if dirp.is_null() {
                break;
            }

            let d_name = CStr::from_ptr((*dirp).d_name.as_ptr()).to_str().unwrap();
            println!("{}", d_name);
        }
        libc::closedir(dp);
        std::process::exit(0);
    }
}
