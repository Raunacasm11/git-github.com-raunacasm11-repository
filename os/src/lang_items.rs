use crate::sbi::shutdown;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    shutdown();
    loop {}
}