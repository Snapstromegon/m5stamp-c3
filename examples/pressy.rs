use std::thread;
use std::time::Duration;

use esp_idf_hal::prelude::Peripherals;
use m5stamp_c3::m5_button::M5Button;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    println!("Hallo Welt!");

    let peripherals = Peripherals::take().expect("Couldn't take peripherals!");
    let button = M5Button::take(peripherals.pins.gpio3)?;

    let mut is_pressed = false;

    loop {
        // println!("Main Loop");
        let current_press = button.is_pressed();
        if is_pressed != current_press {
        if current_press {
            println!("press");
        } else {
            println!("release");
        }
        }
        is_pressed = current_press;
        thread::sleep(Duration::from_millis(100));
    }
}
