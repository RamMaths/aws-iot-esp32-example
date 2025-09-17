pub mod client;
pub mod startup;
use log::*;
use std::time::Duration;
use startup::App;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct JsonMessage {
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // This sets the wifi and creates MQTT client
    let mut app = App::new()?;

    // Start non-blocking message listener
    let message_receiver = app.client.start_message_listener()?;

    // Subscribe to topic
    app.client.subscribe()?;

    info!("Starting main application loop");

    // Main application loop - non-blocking
    loop {
        // Check for MQTT messages without blocking
        match message_receiver.try_recv() {
            Ok(raw_data) => {
                // Try to parse as JSON first
                match serde_json::from_slice::<JsonMessage>(&raw_data) {
                    Ok(msg) => {
                        info!("Received JSON message - action: {}", msg.message);

                        // Handle specific actions
                        let response = match msg.message.as_str() {
                            "ping" => {
                                info!("Ping received, sending pong");
                                JsonMessage {
                                    message: "pong".to_string(),
                                }
                            }
                            _ => {
                                warn!("Unknown action: {}", msg.message);
                                JsonMessage {
                                    message: format!("Unknown action: {}", msg.message),
                                }
                            }
                        };

                        // Send JSON response
                        let json_response = serde_json::to_string(&response)?;
                        app.client.publish(&json_response)?;
                        info!("Sent response: {}", json_response);
                    }
                    Err(_) => {
                        // Fallback for non-JSON messages
                        let message_text = String::from_utf8_lossy(&raw_data);
                        info!("Received non-JSON message: {}", message_text);

                        let response = JsonMessage {
                            message: format!("Received plain text: {}", message_text),
                        };

                        let json_response = serde_json::to_string(&response)?;
                        app.client.publish(&json_response)?;
                    }
                }
            }
            Err(_) => {
                // No message received, continue with other tasks
            }
        }

        // Add any other application logic here

        // Small delay to prevent busy waiting
        std::thread::sleep(Duration::from_millis(100));
    }
}
