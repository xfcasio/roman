#![no_std] #![no_main]
#![feature(never_type)]

use panic_halt as _;
use arduino_hal::port::*;
use arduino_hal::port::mode::Output;
use arduino_hal::{ entry, Peripherals, pins, default_serial, delay_ms };

#[entry]
fn entry() -> ! {
}
