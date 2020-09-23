#![feature(asm, lang_items, unwind_attributes)]
#![no_std]
#![no_main]

extern crate avr_delay;
extern crate avr_std_stub;
extern crate avrd;

use avr_delay::delay;

use avrd::attiny13a::{DDRB, PORTB};

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        core::ptr::write_volatile(DDRB, 0x08);
    }
    let mut out = 0x08;
    loop {
        unsafe {
            core::ptr::write_volatile(PORTB, out);
        }
        delay(1_200_00);
        out ^= 0xFF;
    }
}
