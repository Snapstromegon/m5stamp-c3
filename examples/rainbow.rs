use std::thread;
use std::time::Duration;

use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::timer::EspTimerService;
use m5stamp_c3::{m5_led::M5Led, sk6812::Color};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    println!("Hallo Welt");

    let peripherals = Peripherals::take().expect("Couldn't take peripherals!");
    let mut led = M5Led::take(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    let mut on_off = true;

    let timer = EspTimerService::new()?.timer(move || {
        led.set_color(Color {
            red: if on_off { 255 } else { 0 },
            green: 0,
            blue: 0,
        })
        .expect("Couldn't set color!");
        on_off = !on_off;
    })?;
    timer.every(Duration::from_secs(1))?;

    let mut i = 0;

    loop {
        i = (i+1) % (256*6);
        match i {
            0..=(256 * 1) => 
        }
        thread::sleep(Duration::from_secs(10));
    }
}
