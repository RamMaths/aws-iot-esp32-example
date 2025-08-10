use crate::client::Client;
use embedded_svc::wifi::{ClientConfiguration, Configuration as wifiConfiguration};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};

//Add your wifi credentials in the cfg.toml file
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_pass: &'static str,
    #[default("")]
    mqtt_url: &'static str,
    #[default("")]
    mqtt_client_id: &'static str,
    #[default("")]
    mqtt_topic_pub: &'static str,
    #[default("")]
    mqtt_topic_sub: &'static str,
}

pub struct App {
    pub wifi: EspWifi<'static>,
    pub config: Config,
    pub client: Client,
}

impl App {
    pub fn spawn() -> anyhow::Result<App> {
        let peripherals = unsafe { Peripherals::new() };
        let sys_loop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;
        let app_config: Config = CONFIG;

        let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

        wifi_driver.set_configuration(&wifiConfiguration::Client(ClientConfiguration {
            ssid: app_config.wifi_ssid.try_into().unwrap(),
            password: app_config.wifi_pass.try_into().unwrap(),
            ..Default::default()
        }))?;

        wifi_driver.start()?;
        wifi_driver.connect()?;

        while !wifi_driver.is_connected()? {
            let config = wifi_driver.get_configuration()?;
            log::info!("Waiting for station: {:?}", config);
        }

        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info()?);
        log::info!("Should be connected now with credentials: ");

        let client = Client::new(
            app_config.mqtt_url,
            app_config.mqtt_client_id,
            app_config.mqtt_topic_pub,
            app_config.mqtt_topic_sub,
        )?;

        Ok(App {
            wifi: wifi_driver,
            config: app_config,
            client,
        })
    }
}
