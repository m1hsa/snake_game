use std::mem::zeroed;

const ICANON: u32 = 2;
const ECHO: u32 = 8;
const TCSANOW: i32 = 0;

#[repr(C)]
pub struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}
unsafe extern "C" {
    fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, actions: i32, termios_p: *const Termios) -> i32;
}

pub fn set_term() -> Termios {
    unsafe {
        let mut term: Termios = zeroed();
        tcgetattr(0, &mut term);
        term.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &term);
        term
    }
}

pub fn remove_term(term: Termios) {
    unsafe {
        tcsetattr(0, TCSANOW, &term);
    }
}
