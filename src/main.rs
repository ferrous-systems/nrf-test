#![no_std]
#![no_main]

// Used to define panic behavior
#[allow(unused_imports)]
use panic_halt;

// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
    },
    DWM1001,
};

// Used to set the program entry point
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Access the device hardware
    let mut board = DWM1001::take().unwrap();
    let mut timer = board.TIMER0.constrain();

    // board.leds.D9  - Top LED GREEN
    // board.leds.D12 - Top LED RED
    // board.leds.D11 - Bottom LED RED
    // board.leds.D10 - Bottom LED GREEN

    loop {
        board.leds.D9.enable();
        timer.delay(250_000);

        board.leds.D9.disable();
        timer.delay(750_000);
    }
}
