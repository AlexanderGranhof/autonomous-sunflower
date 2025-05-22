use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Output, PinDriver},
    modem::Modem,
    prelude::Peripherals,
    sys::{EspError, ESP_OK},
};

use crate::pins::{self, Pins};

pub struct Commands<'a> {
    pins: &'a mut Pins,
}

impl<'a> Commands<'a> {
    pub fn new(pins: &'a mut Pins) -> Self {
        Self { pins }
    }

    pub fn turn_led_on(&mut self) {
        self.pins.led.set_high().unwrap();
    }

    pub fn turn_led_off(&mut self) {
        self.pins.led.set_low().unwrap();
    }

    fn led(&mut self, parts: Vec<&str>) -> Result<(), ()> {
        match parts.as_slice() {
            ["led", "on"] => {
                self.pins.led.set_high().unwrap();
                return Ok(());
            }
            ["led", "off"] => {
                self.pins.led.set_low().unwrap();
                return Ok(());
            }
            ["led", "on", time] => {
                self.pins.led.set_high().unwrap();

                FreeRtos::delay_ms(time.parse::<u32>().unwrap());

                self.pins.led.set_low().unwrap();
                return Ok(());
            }
            _ => return Err(()),
        }
    }

    fn pump(&mut self, parts: Vec<&str>) -> Result<(), ()> {
        match parts.as_slice() {
            ["pump", "on", time] => {
                self.pins.water_pump.set_high().unwrap();

                let time = time.parse::<u32>().unwrap().min(3000);

                FreeRtos::delay_ms(time);

                self.pins.water_pump.set_low().unwrap();
                return Ok(());
            }
            _ => return Err(()),
        }
    }

    pub fn parse(&mut self, command: String) -> Result<(), ()> {
        let command = command.trim();
        let parts = command.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "led" => self.led(parts),
            "pump" => self.pump(parts),
            _ => return Err(()),
        }
    }
}
