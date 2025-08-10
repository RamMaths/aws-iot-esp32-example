use esp_idf_svc::{
    mqtt::client::{EspMqttClient, EspMqttConnection, MqttClientConfiguration},
    tls::X509,
};
use std::time::Duration;
use std::{mem, slice};

pub struct Client {
    pub mqtt_client: EspMqttClient<'static>,
    pub mqtt_connection: EspMqttConnection,
    pub pub_topic: String,
    pub sub_topic: String,
}

const SERVER_CERT: &[u8] = include_bytes!("../certs/AmazonRootCA1.pem");
const CLIENT_CERT: &[u8] = include_bytes!("../certs/e5773fe2802720cd400ea6651da78055dbbc5ac58973da1b865c7e778375cbaa-certificate.pem.crt");
const PRIVATE_KEY: &[u8] = include_bytes!("../certs/e5773fe2802720cd400ea6651da78055dbbc5ac58973da1b865c7e778375cbaa-private.pem.key");

impl Client {
    pub fn new(
        url: &str,
        client_id: &str,
        pub_topic: &str,
        sub_topic: &str,
    ) -> anyhow::Result<Client> {
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
            mqtt_connection,
            pub_topic: pub_topic.to_string(),
            sub_topic: sub_topic.to_string(),
        })
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

