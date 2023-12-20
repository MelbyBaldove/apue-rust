use std::{
    ffi::CString,
    io::{self, Write},
};
const MAXLINE: usize = 4096;

fn main() -> io::Result<()> {
    let mut buf = String::with_capacity(MAXLINE);
    let mut status = 0;

    unsafe {
        if libc::signal(libc::SIGINT, sig_int as usize) == libc::SIG_ERR {
            eprintln!("signal error");
        }
        print!("% ");
        io::stdout().flush()?;

        loop {
            buf.clear();
            let bytes_read = io::stdin().read_line(&mut buf)?;
            if bytes_read == 0 {
                break;
            }
            let trimmed = buf.trim_end_matches('\n');
            let buf_c = CString::new(trimmed).unwrap();

            let pid = libc::fork();
            if pid < 0 {
                eprintln!("fork error");
            } else if pid == 0 {
                libc::execlp(buf_c.as_ptr(), buf_c.as_ptr(), 0);
                std::process::exit(127);
            }

            let pid = libc::waitpid(pid, &mut status, 0);
            if pid < 0 {
                eprintln!("wait pid error");
            }
            print!("% ");
            io::stdout().flush()?;
        }
    }
    Ok(())
}

extern "C" fn sig_int(_: i32) {
    println!("interrupt ");
}
