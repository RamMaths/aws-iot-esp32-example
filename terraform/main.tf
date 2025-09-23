provider "aws" {
  region  = var.region
}

# Data source to get current AWS account ID
data "aws_caller_identity" "current" {}

# Create IoT Things using the module
module "iot_things" {
  for_each = { for thing in var.things : thing.name => thing }
  source   = "./thing"

  thing_name         = each.value.name
  topic_prefix       = each.value.topic_prefix
  region             = var.region
  account_id         = var.account_id != "" ? var.account_id : data.aws_caller_identity.current.account_id
  policy_name        = var.policy_name
  certificate_active = var.certificate_active
  tags               = var.tags
}
