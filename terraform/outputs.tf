# Pass through outputs from the things modules
output "things" {
  description = "Details of all created IoT Things"
  value = {
    for name, thing in module.iot_things : name => {
      thing_name        = thing.thing_name
      thing_arn         = thing.thing_arn
      certificate_id    = thing.certificate_id
      certificate_arn   = thing.certificate_arn
      policy_name       = thing.policy_name
      policy_arn        = thing.policy_arn
      iot_endpoint      = thing.iot_endpoint
      mqtt_url          = thing.mqtt_url
      certificate_files = thing.certificate_files
      cfg_toml_template = thing.cfg_toml_template
    }
  }
}

# Individual outputs for backward compatibility
output "iot_endpoint" {
  description = "AWS IoT Core endpoint for MQTT connections"
  value       = length(module.iot_things) > 0 ? values(module.iot_things)[0].iot_endpoint : null
}

# Instructions for next steps
output "next_steps" {
  description = "Instructions for using the created resources"
  value       = <<-EOT

🎉 AWS IoT resources created successfully!

📁 Certificate files downloaded to:
${join("\n", [for name, thing in module.iot_things : "  📁 ${name}:\n    - ${thing.certificate_files.certificate}\n    - ${thing.certificate_files.private_key}\n    - ${thing.certificate_files.public_key}\n    - ${thing.certificate_files.root_ca_1}\n    - ${thing.certificate_files.root_ca_3}"])}

📋 Next steps:
1. Copy certificate files to your firmware projects:
${join("\n", [for name, thing in module.iot_things : "   cp -r certs/${name}/ ../firmware/${name}/"])}

2. Create/update cfg.toml files for each device:
${join("\n\n", [for name, thing in module.iot_things : "   📄 ${name}/cfg.toml:\n${thing.cfg_toml_template}"])}

3. Build and flash your ESP32 firmware for each device:
${join("\n", [for name, thing in module.iot_things : "   cd ../firmware/${name} && cargo build --release && cargo run --release"])}

🌐 MQTT Connection Details:
  - Endpoint: ${length(module.iot_things) > 0 ? values(module.iot_things)[0].iot_endpoint : "N/A"}
  - Things Created: ${join(", ", keys(module.iot_things))}
  - Topics: esp32/* (shared across all devices)

EOT
}