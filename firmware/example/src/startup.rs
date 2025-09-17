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
    #[default("")]
    cert_ca: &'static str,
    #[default("")]
    cert_crt: &'static str,
    #[default("")]
    cert_key: &'static str,
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
        log::info!("  cert_ca: '{}'", self.cert_ca);
        log::info!("  cert_crt: '{}'", self.cert_crt);
        log::info!("  cert_key: '{}'", self.cert_key);
    }
    
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.wifi_ssid.is_empty() {
            return Err("WiFi SSID is empty! Please configure wifi_ssid in cfg.toml".into());
        }
        if self.wifi_pass.is_empty() {
            return Err("WiFi password is empty! Please configure wifi_pass in cfg.toml".into());
        }
        if self.mqtt_url.is_empty() {
            return Err("MQTT URL is empty! Please configure mqtt_url in cfg.toml".into());
        }
        if self.mqtt_client_id.is_empty() {
            return Err("MQTT client ID is empty! Please configure mqtt_client_id in cfg.toml".into());
        }
        if self.mqtt_topic_pub.is_empty() {
            return Err("MQTT publish topic is empty! Please configure mqtt_topic_pub in cfg.toml".into());
        }
        if self.mqtt_topic_sub.is_empty() {
            return Err("MQTT subscribe topic is empty! Please configure mqtt_topic_sub in cfg.toml".into());
        }
        
        log::info!("Configuration validation passed!");
        Ok(())
    }
}

pub struct App {
    pub wifi: EspWifi<'static>,
    pub config: Config,
    pub client: Client,
}

impl App {
    pub fn spawn() -> Result<App, Box<dyn std::error::Error>> {
        let peripherals = unsafe { Peripherals::new() };
        let sys_loop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;
        let app_config: Config = CONFIG;
        app_config.debug_print();
        app_config.validate()?;

        let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

        wifi_driver.set_configuration(&wifiConfiguration::Client(ClientConfiguration {
            ssid: app_config.wifi_ssid.try_into().unwrap(),
            password: app_config.wifi_pass.try_into().unwrap(),
            ..Default::default()
        }))?;

        wifi_driver.start()?;
        log::info!("WiFi started, attempting connection...");
        wifi_driver.connect()?;

        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 30; // 30 seconds timeout
        
        while !wifi_driver.is_connected()? {
            if retry_count >= MAX_RETRIES {
                return Err(format!("WiFi connection timeout after {} seconds", MAX_RETRIES).into());
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
            app_config.mqtt_url,
            app_config.mqtt_client_id,
            app_config.mqtt_topic_pub,
            app_config.mqtt_topic_sub,
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
