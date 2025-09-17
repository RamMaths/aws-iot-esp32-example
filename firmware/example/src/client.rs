use esp_idf_svc::{
    mqtt::client::{EspMqttClient, EspMqttConnection, MqttClientConfiguration, QoS},
    tls::X509,
};
use embedded_svc::mqtt::client::EventPayload::Received;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::time::Duration;
use std::{mem, slice, thread};
use log::*;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct MqttMessage {
    pub action: String,
}

pub struct Client {
    pub mqtt_client: EspMqttClient<'static>,
    pub mqtt_connection: Option<EspMqttConnection>,
    pub pub_topic: String,
    pub sub_topic: String,
    message_sender: Option<Sender<String>>,
}

// Include the generated certificate constants from build.rs
include!(concat!(env!("OUT_DIR"), "/certificates.rs"));

impl Client {
    pub fn new(
        url: &str,
        client_id: &str,
        pub_topic: &str,
        sub_topic: &str,
    ) -> Result<Client, Box<dyn std::error::Error>> {
        log::info!("Loading certificates...");
        log::info!("Server cert size: {} bytes", SERVER_CERT.len());
        log::info!("Client cert size: {} bytes", CLIENT_CERT.len());
        log::info!("Private key size: {} bytes", PRIVATE_KEY.len());

        log::info!("Converting server certificate...");
        let server_cert: X509 = convert_certificate(SERVER_CERT.to_vec());
        log::info!("Server certificate converted successfully");
        
        log::info!("Converting client certificate...");
        let client_cert: X509 = convert_certificate(CLIENT_CERT.to_vec());
        log::info!("Client certificate converted successfully");
        
        log::info!("Converting private key...");
        let private_key: X509 = convert_certificate(PRIVATE_KEY.to_vec());
        log::info!("Private key converted successfully");

        log::info!("Creating MQTT client configuration...");
        
        // AWS IoT requires client certificates for authentication
        let mqtt_client_config = MqttClientConfiguration {
            client_id: Some(client_id),
            crt_bundle_attach: Some(esp_idf_svc::hal::sys::esp_crt_bundle_attach),
            keep_alive_interval: Some(Duration::from_secs(60)),
            server_certificate: Some(server_cert),
            client_certificate: Some(client_cert),
            private_key: Some(private_key),
            ..Default::default()
        };
        log::info!("MQTT client configuration created successfully");

        log::info!("MQTT URL: {}", url);
        log::info!("Creating MQTT client instance...");
        let (mqtt_client, mqtt_connection) = EspMqttClient::new(url, &mqtt_client_config)?;
        log::info!("MQTT client created successfully");

        Ok(Self {
            mqtt_client,
            mqtt_connection: Some(mqtt_connection),
            pub_topic: pub_topic.to_string(),
            sub_topic: sub_topic.to_string(),
            message_sender: None,
        })
    }

    /// Start non-blocking message listener and return a receiver for messages
    pub fn start_message_listener(&mut self) -> Result<Receiver<String>, Box<dyn std::error::Error>> {
        let (tx, rx) = bounded::<String>(10);
        self.message_sender = Some(tx.clone());

        // Take the connection from the Option
        let connection = self.mqtt_connection.take()
            .ok_or("MQTT connection already taken")?;

        thread::Builder::new()
            .stack_size(6000)
            .spawn(move || {
                info!("MQTT message listener started");
                let mut connection = connection;

                while let Ok(event) = connection.next() {
                    info!("[Queue] Event: {}", event.payload());

                    if let Received {
                        id: _,
                        topic: _,
                        data,
                        details: _,
                    } = event.payload()
                    {
                        if let Err(e) = tx.send(String::from_raw_parts(data)) {
                            error!("Failed to send message to channel: {}", e);
                            break;
                        }
                    }
                }

                info!("MQTT message listener stopped");
            })
            .map_err(|e| format!("Failed to spawn message listener thread: {}", e))?;

        Ok(rx)
    }

    /// Subscribe to the configured topic
    pub fn subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.mqtt_client.subscribe(&self.sub_topic, QoS::AtMostOnce) {
                Ok(_) => {
                    info!("Subscribed to topic \"{}\"", self.sub_topic);
                    break;
                }
                Err(e) => {
                    error!("Failed to subscribe to topic \"{}\": {}, retrying...", self.sub_topic, e);
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
        Ok(())
    }

    /// Publish a message to the configured publish topic
    pub fn publish(&mut self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mqtt_client.enqueue(
            &self.pub_topic,
            QoS::AtMostOnce,
            false,
            payload.as_bytes(),
        )?;
        info!("Published \"{}\" to topic \"{}\"", payload, self.pub_topic);
        Ok(())
    }
}

fn convert_certificate(mut certificate_bytes: Vec<u8>) -> X509<'static> {
    // append NUL
    certificate_bytes.push(0);

    // convert the certificate
    let certificate_slice: &[u8] = unsafe {
        let ptr: *const u8 = certificate_bytes.as_ptr();
        let len: usize = certificate_bytes.len();
        mem::forget(certificate_bytes);

        slice::from_raw_parts(ptr, len)
    };

    // return the certificate file in the correct format
    X509::pem_until_nul(certificate_slice)
}

