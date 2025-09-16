provider "aws" {
  region  = var.region
  profile = "ramses"
}

# Data source to get current AWS account ID
data "aws_caller_identity" "current" {}

# Create IoT Thing using the module
module "iot_thing" {
  source = "./thing"

  thing_name         = var.thing_name
  topic_prefix       = var.topic_prefix
  region             = var.region
  account_id         = var.account_id != "" ? var.account_id : data.aws_caller_identity.current.account_id
  policy_name        = var.policy_name
  certificate_active = var.certificate_active
  tags               = var.tags
}
