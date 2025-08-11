fn main() {
    embuild::espidf::sysenv::output();
    
    // Validate that cfg.toml exists and contains required configuration
    let cfg_path = std::path::Path::new("cfg.toml");
    if !cfg_path.exists() {
        panic!("cfg.toml file not found! Please create cfg.toml with your configuration values. See cfg.toml.example for reference.");
    }
    
    // Read and validate config file
    let cfg_content = std::fs::read_to_string(cfg_path)
        .expect("Failed to read cfg.toml file");
    
    // Basic validation - check for required fields
    let required_fields = ["wifi_ssid", "wifi_pass", "mqtt_url", "mqtt_client_id"];
    for field in &required_fields {
        if !cfg_content.contains(field) {
            panic!("cfg.toml is missing required field: {}", field);
        }
    }
    
    println!("cargo:rerun-if-changed=cfg.toml");
    println!("cargo:rustc-env=CONFIG_VALIDATED=1");
}
