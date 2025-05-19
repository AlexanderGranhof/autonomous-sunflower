mod mqtt;
// mod pins;
// mod time;
mod wifi;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use mqtt::Mqtt;
// use pins::Pins;
use wifi::Wifi;

const SSID: &str = env!("SSID");
const SSID_PASSWORD: &str = env!("SSID_PASSWORD");

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    // let pins = Pins::new(peripherals);

    let wifi = Wifi::new(peripherals.modem).connect(SSID, SSID_PASSWORD);

    wifi.wait_netif_up().unwrap();

    let mut mqtt = Mqtt::new("mqtt://broker.emqx.io:1883").topic("mynt/autonomous-sunflower");

    loop {
        FreeRtos::delay_ms(5000);
        mqtt.publish("Hello from ESP32").unwrap();
    }
}
