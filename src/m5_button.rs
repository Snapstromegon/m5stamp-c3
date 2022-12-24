use esp_idf_hal::{
    gpio::{Gpio3, InterruptType, Level},
    peripheral::Peripheral,
    rmt::RmtChannel,
};

use crate::button::Button;

pub struct M5Button<'a> {
    button: Button<'a>,
}

impl<'a> M5Button<'a> {
    pub fn take<C>(
        pin: Gpio3,
        callback: impl FnMut(Level) + Send + 'static,
        interrupt_type: InterruptType,
    ) -> anyhow::Result<Self>
    where
        C: Peripheral + 'a,
        <C as Peripheral>::P: RmtChannel,
    {
        Ok(Self {
            button: Button::new(pin.into(), callback, interrupt_type)?,
        })
    }

    pub fn set_interrupt_type(&mut self, interrupt_type: InterruptType) -> anyhow::Result<()> {
        self.button.set_interrupt_type(interrupt_type)
    }
}
