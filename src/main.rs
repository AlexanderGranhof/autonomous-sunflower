mod commands;
mod mqtt;
mod pins;
mod time;
mod wifi;

use std::time::Duration;

use commands::Commands;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::mqtt::client::{EventPayload, QoS};
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

    let mut mqtt = Mqtt::new("mqtt://broker.emqx.io:1883").topic("mynt/autonomous-sunflower");

    let mynt_topic = "mynt/autonomous-sunflower";

    std::thread::scope(|s| {
        log::info!("About to start the MQTT client");

        // Need to immediately start pumping the connection for messages, or else subscribe() and publish() below will not work
        // Note that when using the alternative constructor - `EspMqttClient::new_cb` - you don't need to
        // spawn a new thread, as the messages will be pumped with a backpressure into the callback you provide.
        // Yet, you still need to efficiently process each message in the callback without blocking for too long.
        //
        // Note also that if you go to http://tools.emqx.io/ and then connect and send a message to topic
        // "esp-mqtt-demo", the client configured here should receive it.
        std::thread::Builder::new()
            .stack_size(6000)
            .spawn_scoped(s, move || {
                log::info!("MQTT Listening for messages");

                while let Ok(event) = mqtt.connection.next() {
                    match event.payload() {
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
                    }
                }
            })
            .unwrap();

        loop {
            if let Err(e) = mqtt.client.subscribe(mynt_topic, QoS::AtMostOnce) {
                log::error!("Failed to subscribe to topic \"{mynt_topic}\": {e}, retrying...");

                // Re-try in 0.5s
                std::thread::sleep(Duration::from_millis(500));

                continue;
            }

            log::info!("Subscribed to topic \"{mynt_topic}\"");

            // Just to give a chance of our connection to get even the first published message
            FreeRtos::delay_ms(500);

            let led_payload = "led on";

            mqtt.client
                .enqueue(mynt_topic, QoS::AtMostOnce, false, led_payload.as_bytes())
                .unwrap();

            let mut delay = 0;

            loop {
                if delay % Time::minutes(1) == 0 {
                    let hours = delay / 3600000;
                    let minutes = (delay % 3600000) / 60000;
                    let seconds = (delay % 60000) / 1000;

                    let message = format!(
                        "Time since last water event: {}h {}m {}s",
                        hours, minutes, seconds
                    );

                    mqtt.client
                        .enqueue(mynt_topic, QoS::AtMostOnce, false, message.as_bytes())
                        .unwrap();
                }

                if delay > Time::hours(24) {
                    log::info!("24 hours passed");
                    let payload = "pump on 2000";

                    mqtt.client
                        .enqueue(mynt_topic, QoS::AtMostOnce, false, payload.as_bytes())
                        .unwrap();
                    delay = 0;
                }

                FreeRtos::delay_ms(Time::seconds(1));
                delay += Time::seconds(1);
            }
        }
    })
}
