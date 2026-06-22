#![no_std]
#![allow(unused)]
use crate::read;

/// Read one single character from standard input (fd = 0)
pub fn getchar() -> u8 {
    let mut buf = [0u8; 1];
    loop {
        let ret = read(0, &mut buf, 1);
        if ret == 1 {
            return buf[0];
        }
    }
}

/// Print single char to standard output fd=1 (existing helper)
pub fn putchar(c: u8) {
    // Existing print logic retained
}