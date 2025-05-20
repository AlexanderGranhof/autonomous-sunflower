use esp_idf_svc::mqtt::client::{EspMqttClient, EspMqttEvent, MqttClientConfiguration, QoS};

pub struct Mqtt<'a> {
    url: &'a str,
    config: MqttClientConfiguration<'a>,
    pub client: Option<EspMqttClient<'a>>,
    topic: Option<&'a str>,
}

impl<'a> Mqtt<'a> {
    pub fn new(url: &'a str) -> Self {
        let config = MqttClientConfiguration {
            client_id: "esp32/automated-sunflower".into(),
            ..MqttClientConfiguration::default()
        };

        return Self {
            config,
            url,
            topic: None,
            client: None,
        };
    }

    pub fn connect<F>(mut self, callback: F) -> Self
    where
        F: for<'b> FnMut(EspMqttEvent<'b>) + Send + 'static,
    {
        let client = EspMqttClient::new_cb(self.url, &self.config, callback).unwrap();

        self.client = Some(client);

        return self;
    }

    pub fn topic(mut self, topic: &'a str) -> Self {
        self.topic = Some(topic);

        return self;
    }

    pub fn subscribe(&mut self) -> Result<(), ()> {
        if let Some(client) = &mut self.client {
            client
                .subscribe(self.topic.unwrap(), QoS::AtMostOnce)
                .unwrap();
        } else {
            log::error!("MQTT client not initialized");
            return Err(());
        }

        return Ok(());
    }

    pub fn publish(&mut self, payload: &str) -> Result<(), ()> {
        if let Some(client) = &mut self.client {
            client
                .enqueue(
                    self.topic.unwrap(),
                    QoS::AtMostOnce,
                    false,
                    payload.as_bytes(),
                )
                .unwrap();
        } else {
            log::error!("MQTT client not initialized");
            return Err(());
        }

        return Ok(());
    }
}
