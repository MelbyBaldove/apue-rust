fn main() {
    unsafe {
        println!("hello world from process ID {}", libc::getpid());
    }
}
