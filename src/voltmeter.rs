// #![no_std]
// #![no_main]

// use core::panic::PanicInfo;
// use esp_backtrace as _;
// use esp_hal::{
//     analog::adc::{Adc, AdcConfig, Attenuation},
//     delay::Delay,
// };

// use esp_println::println;

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

//         let voltage: u32 = sample as u32 * 3300 / 4095;

//         println!("Raw reading: {}, Voltage Reading: {}mV", sample, voltage);

//         delay.delay_millis(500_u32);
//     }
// }
