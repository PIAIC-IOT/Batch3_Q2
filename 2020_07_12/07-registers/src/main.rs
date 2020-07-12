#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln,prelude::*,tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));

    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}

    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (itm, gpioe, rcc, tim6) = aux7::init();

    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());
    // OPM Select one pulse mode
    // CEN Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());


    // Configure the prescaler to have the counter operate at 1 KHz
    // APB1_CLOCK = 8 MHz
    // PSC = 7999
    // 8 MHz / (7999 + 1) = 1 KHz
    // The counter (CNT) will increase on every millisecond
    tim6.psc.write(|w| w.psc().bits(7_999));
    let ms = 50;
    // Turn on the North LED
    gpioe.bsrr.write(|w| w.bs9().set_bit());
    delay(tim6, ms);
    // Turn on the East LED
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    delay(tim6, ms);
    // Turn off the North LED
    gpioe.bsrr.write(|w| w.br9().set_bit());
    delay(tim6, ms);
    // Turn off the East LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}