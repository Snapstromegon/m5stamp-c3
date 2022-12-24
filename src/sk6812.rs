use std::time::Duration;

use anyhow::Result;
use esp_idf_hal::gpio::AnyOutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::TxRmtDriver;
use esp_idf_hal::rmt::{FixedLengthSignal, PinState, Pulse, RmtChannel};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct SK6812<'a> {
    tx: TxRmtDriver<'a>,
    color: Option<Color>,
}

impl<'a> SK6812<'a> {
    pub fn new<C>(pin: AnyOutputPin, channel: C) -> anyhow::Result<Self>
    where
        C: Peripheral + 'a,
        <C as Peripheral>::P: RmtChannel,
    {
        let config = TransmitConfig::new().clock_divider(1);
        let tx = TxRmtDriver::new(channel, pin, &config)?;
        Ok(Self { tx, color: None })
    }

    pub fn set_color(&mut self, color: Color) -> Result<()> {
        // Return early, if color is already correct
        if self
            .color
            .as_ref()
            .map_or(false, |current_color| &color == current_color)
        {
            return Ok(());
        }

        let grb = (color.green as u32) << 16 | (color.red as u32) << 8 | (color.blue as u32);

        let ticks_hz = self.tx.counter_clock()?;
        let true_pulse = (
            Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700))?,
            Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600))?,
        );
        let false_pulse = (
            Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?,
            Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800))?,
        );

        let mut signal = FixedLengthSignal::<24>::new();
        for i in 0..24 {
            let bit = 2_u32.pow(23 - i) & grb != 0;
            let pulse = if bit { true_pulse } else { false_pulse };
            signal.set(i as usize, &pulse)?;
        }
        self.tx.start_blocking(&signal)?;
        self.color = Some(color);
        Ok(())
    }
}
