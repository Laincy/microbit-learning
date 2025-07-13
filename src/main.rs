#![no_std]
#![no_main]

use core::ptr::write_volatile;

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    const GPIO_PINCFG_21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const GPIO_PINCFG_28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const PINCFG_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    unsafe {
        write_volatile(GPIO_PINCFG_21_ROW1_ADDR, PINCFG_DRIVE_LED);
        write_volatile(GPIO_PINCFG_28_COL1_ADDR, PINCFG_DRIVE_LED);
    }

    const GPIO_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const GPIO_OUT_ROW_1_POS: u32 = 21;

    let mut is_on = false;

    loop {
        unsafe {
            write_volatile(GPIO_OUT_ADDR, (is_on as u32) << GPIO_OUT_ROW_1_POS);
        }
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }

        is_on = !is_on
    }
}
