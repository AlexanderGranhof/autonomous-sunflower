use esp_idf_hal::{
    gpio::{Output, PinDriver},
    modem::Modem,
    prelude::Peripherals,
};

pub struct Pins {
    pub water_pump: PinDriver<'static, esp_idf_hal::gpio::Gpio25, Output>,
    pub modem: Modem,
}

impl Pins {
    pub fn new(peripherals: Peripherals) -> Self {
        let water_pump = PinDriver::output(peripherals.pins.gpio25).unwrap();

        Self {
            water_pump,
            modem: peripherals.modem,
        }
    }
}
