use std::thread;
use std::time::Duration;

use esp_idf_hal::{prelude::Peripherals, gpio::InterruptType};
use esp_idf_svc::timer::EspTimerService;
use m5stamp_c3::{m5_led::M5Led, sk6812::Color, m5_button::M5Button};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    println!("Hallo Welt");

    let peripherals = Peripherals::take().expect("Couldn't take peripherals!");
    let mut led = M5Button::take(peripherals.pins.gpio3, ||, InterruptType::AnyEdge)?;

    loop {
        println!("Main Loop");
        thread::sleep(Duration::from_secs(10));
    }
}
