fn main() {
    unsafe {
        println!("uid = {}, gid = {}", libc::getuid(), libc::getgid());
    }
}
