use libc::{ECHO, ICANON, NCCS, TCSANOW, tcgetattr, tcsetattr, termios};
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Save current terminal settings
    let mut oldt = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; NCCS],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    unsafe {
        tcgetattr(fd, &mut oldt);
    }

    // Create a new config with canonical mode and echo turned off
    let mut newt = oldt;
    newt.c_lflag &= !(ICANON | ECHO);

    unsafe {
        tcsetattr(fd, TCSANOW, &newt);
    }

    // Read one character (no Enter required)
    let ch = io::stdin().bytes().next().unwrap()?; // read 1 byte
    println!("\nYou pressed: {}", ch as char);

    // Restore terminal settings
    unsafe {
        tcsetattr(fd, TCSANOW, &oldt);
    }

    Ok(())
}
