#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    timer::{timg::TimerGroup, Timer},
};
use esp_println::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let output_config = OutputConfig::default();
    let mut led_pin = Output::new(peripherals.GPIO1, Level::Low, output_config);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0);
    let timer0 = timer_group0.timer0;
    let mut start = timer0.now();
    timer0.start();
    loop {
        if start.elapsed().as_secs() >= 1 {
            led_pin.toggle();
            start = timer0.now();
        }
    }
}
