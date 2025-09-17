variable "thing_name" {
  description = "Name of the IoT Thing to create"
  type        = string
  default     = "my-esp32-device"
}

variable "topic_prefix" {
  description = "Topic prefix for MQTT topics (replaces 'esp32' in policy)"
  type        = string
  default     = "esp32"
}

variable "region" {
  description = "AWS region where IoT resources will be created"
  type        = string
  default     = "us-east-1"
}

variable "account_id" {
  description = "AWS account ID (leave empty to auto-detect)"
  type        = string
  default     = ""
}

variable "policy_name" {
  description = "Custom name for the IoT policy (optional)"
  type        = string
  default     = null
}

variable "certificate_active" {
  description = "Whether the IoT certificate should be active"
  type        = bool
  default     = true
}

variable "tags" {
  description = "Tags to apply to all resources"
  type        = map(string)
  default = {
    Project     = "ESP32-IoT-Example"
    Environment = "development"
    ManagedBy   = "terraform"
  }
}