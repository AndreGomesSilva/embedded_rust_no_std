// #![no_std]
// #![no_main]

// use core::panic::PanicInfo;
// use esp_backtrace as _;
// use esp_hal::{
//     analog::adc::{Adc, AdcConfig, Attenuation},
//     delay::Delay,
// };
// use esp_println::println;
// use libm::log;

// const B: f64 = 3950.0;
// const VMAX: f64 = 4095.0;

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }

// #[esp_hal::main]
// fn main() -> ! {
//     let peripherals = esp_hal::init(esp_hal::Config::default());
//     let delay = Delay::new();
//     let mut adc_config = AdcConfig::new();

//     //configure ADC channel
//     let mut adc_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);

//     let mut adc = Adc::new(peripherals.ADC1, adc_config);

//     loop {
//         let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();

//         let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;

//         println!("Temperature {:02} Celcius \r", temperature);

//         delay.delay_millis(500_u32);
//     }
// }
