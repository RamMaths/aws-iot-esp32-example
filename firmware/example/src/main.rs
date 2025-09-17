pub mod client;
pub mod startup;
use log::*;
use std::time::Duration;
use startup::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // This sets the wifi and creates MQTT client
    let mut app = App::spawn()?;

    // Start non-blocking message listener
    let message_receiver = app.client.start_message_listener()?;

    // Subscribe to topic
    app.client.subscribe()?;

    info!("Starting main application loop");

    // Main application loop - non-blocking
    loop {
        // Check for MQTT messages without blocking
        match message_receiver.try_recv() {
            Ok(message) => {
                info!("Received message: {}", message);
                let payload = format!("Message received in esp32: {}", message);
                app.client.publish(&payload)?;
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

