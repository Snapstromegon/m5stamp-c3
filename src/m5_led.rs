use esp_idf_hal::{gpio::Gpio2, peripheral::Peripheral, rmt::RmtChannel};

pub use crate::sk6812::Color;
use crate::sk6812::SK6812;

pub struct M5Led<'a> {
    led: SK6812<'a>,
}

impl<'a> M5Led<'a> {
    pub fn take<C>(pin: Gpio2, channel: C) -> anyhow::Result<Self>
    where
        C: Peripheral + 'a,
        <C as Peripheral>::P: RmtChannel,
    {
        Ok(Self {
            led: SK6812::new(pin.into(), channel)?,
        })
    }

    pub fn set_color(&mut self, color: Color) -> anyhow::Result<()> {
        self.led.set_color(color)
    }
}
