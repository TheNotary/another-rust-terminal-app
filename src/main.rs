use libc::{ECHO, ICANON, TCSANOW, tcgetattr, tcsetattr, termios};
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use std::mem;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Save current terminal settings
    let mut oldt: termios = unsafe { mem::zeroed() };
    unsafe {
        tcgetattr(fd, &mut oldt);
    }

    // Create a new config with canonical mode and echo turned off
    let mut newt = oldt;
    newt.c_lflag &= !(ICANON | ECHO);

    unsafe {
        tcsetattr(fd, TCSANOW, &newt);
    }

    // Block here until the next character to read from stdin is pressed
    let ch = io::stdin().bytes().next().unwrap()?; // read 1 byte

    println!("\nYou pressed: {}", ch as char);

    // Restore terminal settings
    unsafe {
        tcsetattr(fd, TCSANOW, &oldt);
    }

    Ok(())
}
