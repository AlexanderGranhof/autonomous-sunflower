mod commands;
mod mqtt;
mod pins;
mod time;
mod wifi;

use commands::Commands;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::mqtt::client::EventPayload;
use mqtt::Mqtt;
use pins::Pins;
use time::Time;
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

    let mut pins = Pins::new(peripherals.pins);

    let wifi = Wifi::new(peripherals.modem).connect(SSID, SSID_PASSWORD);

    wifi.wait_netif_up().unwrap();

    let mut parser = Commands::new(&mut pins);

    // let mut mqtt = Mqtt::new("mqtt://broker.emqx.io:1883")
    let mut mqtt = Mqtt::new("mqtt://10.0.0.135:1883")
        .topic("mynt/autonomous-sunflower")
        .connect(move |event| match event.payload() {
            EventPayload::Connected(true) => {
                log::info!("Connected to MQTT broker");
            }
            EventPayload::Connected(false) => {
                log::error!("Failed to connect to MQTT broker");
            }
            EventPayload::Disconnected => {
                log::info!("Disconnected from MQTT broker");
            }
            EventPayload::Received {
                id,
                topic,
                data,
                details,
            } => {
                let command = String::from_utf8(data.into()).unwrap_or("".to_string());

                match parser.parse(command) {
                    Ok(_) => log::info!("Command executed successfully"),
                    _ => (),
                }
            }
            EventPayload::BeforeConnect => {
                log::info!("Before connect");
            }
            _ => {
                log::info!("Event: {:?}", event.payload());
            }
        });

    let mut delay = 0;

    loop {
        FreeRtos::delay_ms(1000);

        if delay % 5000 == 0 {
            let hours = delay / 3600000;
            let minutes = (delay % 3600000) / 60000;
            let seconds = (delay % 60000) / 1000;

            let message = format!(
                "Time since last water event: {}h {}m {}s",
                hours, minutes, seconds
            );

            mqtt.publish(&message).unwrap();
        }

        if delay > Time::seconds(20) {
            log::info!("20 seconds passed");
            parser2.turn_led_on();
            FreeRtos::delay_ms(1000);
            parser2.turn_led_off();
            delay = 0;
        }

        if let Err(()) = mqtt.subscribe() {
            log::error!("Failed to subscribe to topic");
            FreeRtos::delay_ms(1000);

            continue;
        } else {
            log::info!("Subscribed to topic");
        }

        delay += 1000;
    }
}
