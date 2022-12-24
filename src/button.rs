use esp_idf_hal::gpio::{AnyInputPin, Input, InterruptType, Level, PinDriver};

pub struct Button<'d> {
    pin_driver: PinDriver<'d, AnyInputPin, Input>,
}

impl<'d> Button<'d> {
    pub fn new(
        pin: AnyInputPin,
        callback: impl FnMut(Level) + Send + 'static,
        interrupt_type: InterruptType,
    ) -> anyhow::Result<Self> {
        let mut pd = PinDriver::input(pin)?;
        unsafe {
            pd.subscribe(|| callback(pd.get_level()))?;
        }
        pd.set_interrupt_type(interrupt_type)?;
        Ok(Self { pin_driver: pd })
    }

    pub fn set_interrupt_type(&mut self, interrupt_type: InterruptType) -> anyhow::Result<()> {
        self.pin_driver.set_interrupt_type(interrupt_type)?;
        Ok(())
    }
}
