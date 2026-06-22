#![no_std]
#![no_main]
use user_lib::{getchar, fork, exec, waitpid, heap_init, String, exit};

#[no_mangle]
fn main() -> i32 {
    heap_init();
    let mut input_buf = String::new();
    loop {
        // Print shell prompt
        print!(">> ");
        input_buf.clear();
        loop {
            let ch = getchar();
            match ch {
                // Enter key: execute command
                b'\n' => {
                    if !input_buf.is_empty() {
                        let pid = fork();
                        if pid == 0 {
                            // Child run target program
                            exec(&input_buf);
                            exit(-1);
                        } else {
                            // Parent wait for child finish
                            let mut exit_code = 0;
                            waitpid(pid, &mut exit_code);
                        }
                    }
                    break;
                }
                // Backspace key: erase last character
                127 => {
                    if input_buf.len() > 0 {
                        input_buf.pop();
                    }
                }
                // Normal printable character
                32..=126 => input_buf.push(ch as char),
                _ => (),
            }
        }
    }
}