// no_std attribute disable automatic include std
#![no_std]
// disable runtime entry
#![no_main]

mod vga_buffer;

// panic will run this
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
