
#![no_std]
#![no_main]

extern crate panic_halt;
use riscv_rt::entry;
use bl808_pac::Peripherals;

#[entry]
fn main() -> ! {

    let p = unsafe { Peripherals::steal() };

    p.GLB.gpio_config[8].modify(|_, w| w
        .pin_mode().output()
        .input_function().clear_bit()
        .output_function().set_bit()
        .pull_up().clear_bit()
        .pull_down().set_bit()
        .output_value().set_bit()
    );

    loop {}
}
