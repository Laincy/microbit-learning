#![no_std]
#![no_main]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world");
    loop {
        rprintln!("Echo ...");
        for _ in 0..100_000 {
            cortex_m::asm::nop();
        }
    }
}
