#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;
use riscv_rt as _;

mod math_model;
use math_model::{HuffmanDecoder, SourceSymbol};
use riscv_semihosting::hprintln;

#[no_mangle]
pub unsafe fn _critical_section_1_0_acquire() -> u8 {
    let mstatus: usize;
    core::arch::asm!("csrrci {0}, mstatus, 8", out(reg) mstatus);
    ((mstatus & 8) >> 3) as u8
}

#[no_mangle]
pub unsafe fn _critical_section_1_0_release(was_enabled: u8) {
    if was_enabled != 0 {
        riscv::interrupt::enable();
    }
}

#[entry]
fn main() -> ! {
    let bitstream = [true, false, true, true, false];

    let mut decoder = HuffmanDecoder::new(&bitstream);

    hprintln!("--- Starting Decode ---");
    loop {
        let symbol = decoder.decode_next();

        if symbol == SourceSymbol::DecodeError {
            hprintln!("Decode finished or error encountered.");
            break;
        }

        hprintln!("Captured Symbol: {:?}", symbol);
    }

    hprintln!("Execution complete. Entering infinite loop.");
    loop {}
}
