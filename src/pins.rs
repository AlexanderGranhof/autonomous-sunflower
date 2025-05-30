use esp_idf_hal::gpio::{Output, PinDriver, Pins as EspPins};

pub struct Pins {
    pub water_pump: PinDriver<'static, esp_idf_hal::gpio::Gpio12, Output>,
    pub led: PinDriver<'static, esp_idf_hal::gpio::Gpio23, Output>,
}

impl Pins {
    pub fn new(pins: EspPins) -> Self {
        let water_pump = PinDriver::output(pins.gpio12).unwrap();
        let led = PinDriver::output(pins.gpio23).unwrap();

        Self { water_pump, led }
    }

    pub fn get_gpio23(&mut self) -> &PinDriver<'static, esp_idf_hal::gpio::Gpio23, Output> {
        return &self.led;
    }
}
