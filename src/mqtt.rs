use esp_idf_hal::sys::EspError;
use esp_idf_svc::mqtt::client::{
    EspMqttClient, EspMqttConnection, EspMqttEvent, MqttClientConfiguration, QoS,
};

pub struct Mqtt<'a> {
    pub client: EspMqttClient<'a>,
    pub connection: EspMqttConnection,
    topic: Option<&'a str>,
}

impl<'a> Mqtt<'a> {
    pub fn new(url: &'a str) -> Self {
        let config = MqttClientConfiguration {
            client_id: "esp32/automated-sunflower".into(),
            ..MqttClientConfiguration::default()
        };

        let (client, connection) = EspMqttClient::new(url, &config).unwrap();

        return Self {
            topic: None,
            client,
            connection,
        };
    }

    // pub fn connect<F>(mut self, callback: F) -> Self
    // where
    //     F: for<'b> FnMut(EspMqttEvent<'b>) + Send + 'static,
    // {
    //     let client = EspMqttClient::new_cb(self.url, &self.config, callback).unwrap();

    //     self.client = Some(client);

    //     return self;
    // }

    pub fn topic(mut self, topic: &'a str) -> Self {
        self.topic = Some(topic);

        return self;
    }

    pub fn subscribe(&mut self) -> Result<(), ()> {
        self.client.subscribe(self.topic.unwrap(), QoS::AtMostOnce);

        return Ok(());
    }

    pub fn publish(&mut self, payload: &str) -> Result<(), ()> {
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
