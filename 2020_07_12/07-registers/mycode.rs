#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        const GPIOE_BSRR: u32 = 0x4800_1018;
        const GPIOE_ODR: u32 = 0x4800_1014;

                // Turn on the NORTH LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0b{:b}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = {}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn on the EAST LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0b{:b}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = {}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the NORTH LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0b{:b}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = {}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        // Turn off the EAST LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0b{:b}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );

        iprintln!(
            &mut itm.stim[0],
            "ODR = {}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }

    loop {}
}