mod commands;
mod controller;
mod mqtt;
mod pins;
mod time;
mod wifi;

use controller::Board;
use controller::BoardController;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use pins::Pins;
use time::Time;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut pins = Pins::new(peripherals.pins);

    let mut controller = Board::new(&mut pins);

    loop {
        controller.led(true);

        let mut delay = 0;

        loop {
            if delay % Time::seconds(1) == 0 {
                let hours = delay / 3600000;
                let minutes = (delay % 3600000) / 60000;
                let seconds = (delay % 60000) / 1000;

                log::info!(
                    "Time since last water event: {}h {}m {}s",
                    hours,
                    minutes,
                    seconds
                );
            }

            if delay > Time::hours(24) {
                log::info!("24 hours passed");
                controller.pump(2000);

                delay = 0;
            }

            FreeRtos::delay_ms(Time::seconds(1));
            delay += Time::seconds(1);
        }
    }
}
