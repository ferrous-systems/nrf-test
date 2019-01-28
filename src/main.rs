#![no_std]
#![no_main]

// Used to define panic behavior
#[allow(unused_imports)]
use panic_halt;

// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
        uarte::Error as UErr,
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

    let out: [u8; 6] = [
        0x70, // P
        0x69, // I
        0x6e, // N
        0x67, // G
        0x0d, // CR
        0x0a  // LF
    ];

    let mut buf = [0u8; 1];

    loop {
        board.leds.D9.enable();
        timer.delay(10_000);
        board.leds.D9.disable();

        match board.uart.read_timeout(
            &mut buf,
            &mut timer,
            5_000_000) {
            Ok(_) => {
                board.uart.write(&buf).unwrap();
            }
            Err(UErr::Timeout(n)) if n > 0 => {
                board.uart.write(&buf[..n]).unwrap();
                board.uart.write(&out[4..]).unwrap();
                board.uart.write(&out).unwrap();
            }
            Err(_) => {
                panic!()
            }
        };
    }
}
