use std::{
    env,
    ffi::CString,
    io::{self, Write},
    mem::MaybeUninit,
};

use libc::{c_char, S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK};

fn main() {
    let mut args = env::args();
    args.next();

    unsafe {
        let mut buf: libc::stat = MaybeUninit::zeroed().assume_init();
        for arg in args {
            print!("{}:", arg);
            io::stdout().flush().unwrap();
            let arg_c = CString::new(arg).unwrap();
            if libc::lstat(arg_c.as_ptr() as *const c_char, &mut buf) < 0 {
                eprintln!("lstat error");
                continue;
            }

            let ptr = match buf.st_mode & S_IFMT {
                S_IFREG => String::from("regular"),
                S_IFDIR => String::from("directory"),
                S_IFCHR => String::from("character special"),
                S_IFBLK => String::from("block special"),
                S_IFIFO => String::from("fifo"),
                S_IFLNK => String::from("symbolic link"),
                S_IFSOCK => String::from("socket"),
                _ => String::from("** unknown mode **"),
            };
            println!("{}", ptr);
        }
    }

    std::process::exit(0);
}
