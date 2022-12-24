use esp_idf_hal::gpio::{AnyIOPin, Input, InterruptType, Level, PinDriver, Pull};

pub struct Button<'d> {
    pin_driver: PinDriver<'d, AnyIOPin, Input>,
}

impl<'d> Button<'d> {
    pub fn new(pin: AnyIOPin) -> anyhow::Result<Self> {
        let mut pd = PinDriver::input(pin)?;
        pd.set_pull(Pull::Up)?;
        Ok(Self { pin_driver: pd })
    }

    pub fn set_interrupt_type(&mut self, interrupt_type: InterruptType) -> anyhow::Result<()> {
        self.pin_driver.set_interrupt_type(interrupt_type)?;
        Ok(())
    }

    pub fn is_pressed(&self) -> bool {
        self.pin_driver.get_level() == Level::Low
    }
}
