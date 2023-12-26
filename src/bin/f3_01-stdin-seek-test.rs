fn main() {
    unsafe {
        if libc::lseek(libc::STDIN_FILENO, 0, libc::SEEK_CUR) == -1 {
            println!("cannot seek");
        } else {
            println!("seek OK");
        }
    }
    std::process::exit(0);
}
