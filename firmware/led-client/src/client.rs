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

const SERVER_CERT: &[u8] = include_bytes!(env!("SERVER_CERT_PATH"));
const CLIENT_CERT: &[u8] = include_bytes!(env!("CLIENT_CERT_PATH"));
const PRIVATE_KEY: &[u8] = include_bytes!(env!("PRIVATE_KEY_PATH"));

impl Client {
    pub fn new(
        url: &str,
        client_id: &str,
        pub_topic: &str,
        sub_topic: &str,
        server_cert_path: &str,
        client_cert_path: &str,
        private_key_path: &str,
    ) -> anyhow::Result<Client> {
        let server_cert: X509 = convert_static_certificate(server_cert_bytes);
        let client_cert: X509 = convert_static_certificate(client_cert_bytes);
        let private_key: X509 = convert_static_certificate(private_key_bytes);

        let mqtt_client_config = MqttClientConfiguration {
            client_id: Some(client_id),
            crt_bundle_attach: Some(esp_idf_svc::hal::sys::esp_crt_bundle_attach),
            keep_alive_interval: Some(Duration::from_secs(60)),
            server_certificate: Some(server_cert),
            client_certificate: Some(client_cert),
            private_key: Some(private_key),
            ..Default::default()
        };

        let (mqtt_client, mqtt_connection) = EspMqttClient::new(url, &mqtt_client_config)?;

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

fn convert_static_certificate(c: &'static [u8]) -> X509<'static> {
    // append NUL
    let mut certificate_bytes = c.to_vec();
    certificate_bytes.push(0);

    // convert the certificate
    let certificate_bytes_slice = unsafe {
        let ptr: *const u8 = certificate_bytes.as_ptr();
        let len: usize = certificate_bytes.len();
        std::mem::forget(certificate_bytes);
        std::slice::from_raw_parts(ptr, len)
    };

    // return the certificate file in the correct format
    X509::pem_until_nul(certificate_bytes_slice)
}
