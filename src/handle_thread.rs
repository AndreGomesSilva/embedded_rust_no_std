// #![no_std]
// #![no_main]


// use esp_backtrace as _;
// use core::cell::RefCell;
// use core::panic::PanicInfo;
// use critical_section::Mutex;
// use esp_hal::{delay::Delay, gpio::{Io, Event, Input, InputConfig, Pull}, handler, main};
// use esp_println::println;


// static G_PIN: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop{}
// }

// #[handler]
// fn gpio_interrupt_handler(){
//     critical_section::with(|cs| {
//         if let Some(pin) = G_PIN.borrow_ref_mut(cs).as_mut() {
//             if pin.is_interrupt_set() {
//                 println!("Interrupt detect on GPIO");
//                 pin.clear_interrupt();
//             }
//         }
//     });
// }

// #[main]
// fn main() -> ! {
//     let peripherals = esp_hal::init(esp_hal::Config::default());

//     let mut io = Io::new(peripherals.IO_MUX);

//     io.set_interrupt_handler(gpio_interrupt_handler);

//     let config = InputConfig::default().with_pull(Pull::Up);
//     let mut some_pin = Input::new(peripherals.GPIO0, config);

//     some_pin.listen(Event::FallingEdge);

//     critical_section::with(|cs| {
//         G_PIN.borrow_ref_mut(cs).replace(some_pin);
//     });

//     let delay = Delay::new();

//     loop {
//         println!("loop principal rodando...");
//         delay.delay_millis(1000u32);
//     }
// }