// #![no_std]
// #![no_main]


// use esp_backtrace as _;
// use core::panic::PanicInfo;
// use esp_println::println;
// use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig}};

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop{}
// }

// #[esp_hal::main]
// fn main() -> ! {
//     let peripherals = esp_hal::init(esp_hal::Config::default());

//     let output_config = OutputConfig::default();

//     let mut led_pin = Output::new(peripherals.GPIO1,Level::Low,  output_config);

//     let delay = Delay::new();

//     loop {
//         led_pin.set_high();

//         delay.delay_millis(1000u32);

//         led_pin.set_low();

//         delay.delay_millis(1000u32);
//     }
// }