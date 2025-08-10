use crate::client::Client;
use embedded_svc::wifi::{ClientConfiguration, Configuration as wifiConfiguration};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::EspWifi};
use std::time::Duration;
use std::thread;

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

// Add debug logging for config values
impl Config {
    pub fn debug_print(&self) {
        log::info!("Config values:");
        log::info!("  wifi_ssid: '{}'", self.wifi_ssid);
        log::info!("  wifi_pass: '{}'", if self.wifi_pass.is_empty() { "EMPTY" } else { "SET" });
        log::info!("  mqtt_url: '{}'", self.mqtt_url);
        log::info!("  mqtt_client_id: '{}'", self.mqtt_client_id);
        log::info!("  mqtt_topic_pub: '{}'", self.mqtt_topic_pub);
        log::info!("  mqtt_topic_sub: '{}'", self.mqtt_topic_sub);
    }
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
        app_config.debug_print();

        let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

        wifi_driver.set_configuration(&wifiConfiguration::Client(ClientConfiguration {
            ssid: "INFINITUM450B".try_into().unwrap(),
            password: "dn2PuRUEHt".try_into().unwrap(),
            ..Default::default()
        }))?;

        wifi_driver.start()?;
        log::info!("WiFi started, attempting connection...");
        wifi_driver.connect()?;

        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 30; // 30 seconds timeout
        
        while !wifi_driver.is_connected()? {
            if retry_count >= MAX_RETRIES {
                return Err(anyhow::anyhow!("WiFi connection timeout after {} seconds", MAX_RETRIES));
            }
            
            let config = wifi_driver.get_configuration()?;
            log::info!("Waiting for station (attempt {}): {:?}", retry_count + 1, config);
            
            // Feed the watchdog and add delay
            unsafe {
                esp_idf_svc::hal::sys::esp_task_wdt_reset();
            }
            thread::sleep(Duration::from_secs(1));
            retry_count += 1;
        }

        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info()?);
        log::info!("Should be connected now with credentials: ");

        log::info!("Creating MQTT client...");
        let client = match Client::new(
            "mqtts://d044673527boztw2638hx-ats.iot.us-east-1.amazonaws.com",
            "esp32s3",
            "topic/pub",
            "topic/sub",
        ) {
            Ok(client) => {
                log::info!("MQTT client created successfully");
                client
            }
            Err(e) => {
                log::error!("Failed to create MQTT client: {:?}", e);
                return Err(e);
            }
        };

        Ok(App {
            wifi: wifi_driver,
            config: app_config,
            client,
        })
    }
}
