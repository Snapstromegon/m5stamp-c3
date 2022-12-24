use esp_idf_hal::gpio::Gpio3;

use crate::button::Button;

pub struct M5Button<'a> {
    button: Button<'a>,
}

impl<'a> M5Button<'a> {
    pub fn take(pin: Gpio3) -> anyhow::Result<Self> {
        Ok(Self {
            button: Button::new(pin.into())?,
        })
    }

    pub fn is_pressed(&self) -> bool {
        self.button.is_pressed()
    }
}
