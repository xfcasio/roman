#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[avr_device::entry]
fn main() -> ! {
    unsafe { 
        core::ptr::write_volatile(0x24 as *mut u8, 1 << 5);

        loop {
            core::ptr::write_volatile(0x25 as *mut u8, 1 << 5);
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
     loop {}
}
