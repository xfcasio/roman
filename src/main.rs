#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::port::*;
use arduino_hal::port::mode::Output;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut d13 = pins.d13.into_output();
    
    for freq in 1..100 {
        beep(&mut d13, 100*freq, freq.try_into().unwrap());
    }

    loop {}
}

fn beep<P: PinOps>(pin: &mut Pin<Output, P>, dur: u32, freq: u16) {
    for _ in 0..dur {
        pin.toggle();
        arduino_hal::delay_ms(freq);
    }
}

