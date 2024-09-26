#![no_std] #![no_main]
#![feature(never_type)]

use panic_halt as _;
use arduino_hal::port::*;
use arduino_hal::port::mode::Output;
use arduino_hal::{ entry, Peripherals, pins, default_serial, delay_ms };

#[entry]
fn entry() -> ! {
}

fn beep<P: PinOps>(pin: &mut Pin<Output, P>, duration: u32, freq: u16) {
    for _ in 0..duration {
        pin.toggle();
        delay_ms(freq);
    }
}
