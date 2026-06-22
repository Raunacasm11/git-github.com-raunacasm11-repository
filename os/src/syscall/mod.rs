pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        64 => sys_write(args[0], args[1] as *const u8, args[2]),
        169 => sys_get_time(),
        _ => -1,
    }
}

fn sys_write(_fd: usize, buf: *const u8, len: usize) -> isize {
    let s = unsafe { core::slice::from_raw_parts(buf, len) };
    for &b in s {
        crate::sbi::console_putchar(b as usize);
    }
    len as isize
}

fn sys_get_time() -> isize {
    crate::timer::get_time_ms() as isize
}