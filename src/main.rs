#![no_std]
#![no_main]

// Used to define panic behavior
#[allow(unused_imports)]
use panic_halt;

// Used to wait for packet to send
use nb::block;

// Used to set the program entry point
use cortex_m_rt::entry;

// Provides definitions for our development board
use dwm1001::{
    nrf52832_hal::{
        prelude::*,
        Delay,
    },
    dw1000::{
        mac,
    },

    DWM1001,
};


#[entry]
fn main() -> ! {
    let mut board  = DWM1001::take().unwrap();
    let mut timer  = board.TIMER0.constrain();
    let mut rng    = board.RNG.constrain();
    let     clocks = board.CLOCK.constrain().freeze();
    let mut delay  = Delay::new(board.SYST, clocks);

    board.DW_RST.reset_dw1000(&mut delay);
    let mut dw1000 = board.DW1000.init()
        .expect("Failed to initialize DW1000");

    // TODO! Configure Radio!

    let mut toggle = false;

    loop {

        // TODO! Send Data!
        // let mut tx = dw1000
        //     .send()
        //     .expect("Failed to start receiver");

        // block!(tx.wait())
        //     .expect("Failed to send data");

        if toggle {
            board.leds.D10.enable();
        } else {
            board.leds.D10.disable();
        }

        toggle = !toggle;

        timer.delay(1_000_000);
    }
}
