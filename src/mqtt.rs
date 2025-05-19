use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration, QoS};

pub struct Mqtt<'a> {
    client: EspMqttClient<'a>,
    topic: Option<&'a str>,
}

impl<'a> Mqtt<'a> {
    pub fn new(url: &str) -> Self {
        let config = MqttClientConfiguration {
            client_id: "esp32/automated-sunflower".into(),
            server_certificate: None,
            skip_cert_common_name_check: true,
            ..MqttClientConfiguration::default()
        };

        let (client, _) = EspMqttClient::new(url, &config).unwrap();

        log::info!("Connected to MQTT");

        return Self {
            client,
            topic: None,
        };
    }

    pub fn topic(mut self, topic: &'a str) -> Self {
        self.topic = Some(topic);

        return self;
    }

    pub fn publish(&mut self, payload: &str) -> Result<(), ()> {
        if self.topic.is_none() {
            log::info!("Topic is not set");
            return Err(());
        }

        self.client
            .enqueue(
                self.topic.unwrap(),
                QoS::AtMostOnce,
                false,
                payload.as_bytes(),
            )
            .unwrap();

        return Ok(());
    }
}
