use esp_idf_hal::delay::FreeRtos;

use crate::pins::Pins;

pub trait BoardController<'a> {
    fn new(pins: &'a mut Pins) -> Self;
    fn pump(&mut self, duration: u32);
    fn led(&mut self, on: bool);
}

// impl Board {
//     pub fn new(pins: Pins) -> Self {
//         Self { pins }
//     }
// }

pub struct Board<'a> {
    pins: &'a mut Pins,
}

impl<'a> BoardController<'a> for Board<'a> {
    fn new(pins: &'a mut Pins) -> Self {
        Self { pins }
    }

    fn pump(&mut self, duration: u32) {
        self.pins.water_pump.set_high().unwrap();

        FreeRtos::delay_ms(duration);

        self.pins.water_pump.set_low().unwrap();
    }

    fn led(&mut self, on: bool) {
        // Implementation for LED control
        // TODO: Add actual LED control logic using self.pins
    }
}

pub struct Controller<'a> {
    board: &'a mut Board<'a>,
}

impl<'a> Controller<'a> {
    pub fn new(board: &'a mut Board<'a>) -> Self {
        Self { board }
    }

    pub fn pump(&mut self, duration: u32) {
        self.board.pump(duration);
    }

    pub fn led(&mut self, on: bool) {
        self.board.led(on);
    }
}
