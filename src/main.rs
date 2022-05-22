#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate rtt_target;

extern crate embedded_hal;
extern crate embedded_time;

extern crate stm32f1xx_hal;

extern crate panic_probe;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut flash = pac.FLASH.constrain();
    let rcc = pac.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(72.MHz()).freeze(&mut flash.acr);
    
    let mut delay = core.SYST.delay(&clocks);

    let mut gpioc = pac.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        rprintln!("blink");
        led.set_low();
        delay.delay_ms(100u16);
        led.set_high();
        delay.delay_ms(100u16);
        led.set_low();
        delay.delay_ms(100u16);
        led.set_high();
        delay.delay_ms(700u16);
    }
}
