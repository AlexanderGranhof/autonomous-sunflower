use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};

pub struct Wifi<'a> {
    wifi: BlockingWifi<EspWifi<'a>>,
}

impl<'a> Wifi<'a> {
    pub fn new(modem: Modem) -> Self {
        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();

        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(modem, sys_loop.clone(), Some(nvs)).unwrap(),
            sys_loop,
        )
        .unwrap();

        Self { wifi }
    }

    pub fn connect(mut self, ssid: &str, password: &str) -> BlockingWifi<EspWifi<'a>> {
        let wifi_config = Configuration::Client(ClientConfiguration {
            ssid: ssid.try_into().unwrap(),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: password.try_into().unwrap(),
            channel: None,
            ..Default::default()
        });

        self.wifi.set_configuration(&wifi_config).unwrap();

        self.wifi.start().unwrap();
        self.wifi.connect().unwrap();

        return self.wifi;
    }
}
