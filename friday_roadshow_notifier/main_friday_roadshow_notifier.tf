locals {
  slack_channel_id_name = "friday_roadshow_notifier_slack_channel_id"
  slack_token_name      = "friday_roadshow_notifier_slack_token"
  parameters = {
    "${local.slack_channel_id_name}" = {
      type                 = "SecureString"
      value                = "dummy_value"
      description          = "Format: 11-digit code starting with C. For friday_roadshow_notifier."
      ignore_value_changes = true
    }
    "${local.slack_token_name}" = {
      type                 = "SecureString"
      value                = "dummy_value"
      description          = "Format: ([a-z]*4)-([0-9]*13)-([0-9]*13)-([a-zA-Z0-9]*24)`). For bot access for friday_roadshow_notifier."
      ignore_value_changes = true
    }
  }
}

module "ssm_params" {
  source = "terraform-aws-modules/ssm-parameter/aws"

  for_each = local.parameters

  name                 = try(each.value.name, each.key)
  value                = try(each.value.value, null)
  type                 = try(each.value.type, null)
  description          = try(each.value.description, null)
  ignore_value_changes = try(each.value.ignore_value_changes, false)
}

module "event_bridge" {
  source = "terraform-aws-modules/eventbridge/aws"

  create_bus = false

  rules = {
    friday_roadshow_notifier = {
      state = "ENABLED"
      schedule_expression = "cron(30 03 * ? 3 *)"
    }
  }

  targets = {
    friday_roadshow_notifier = [
      {
        name = "friday-roadshow-notifier-lambda"
        arn  = module.lambda_function.lambda_function_arn
      }
    ]
  }
}

module "lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "friday-roadshow-notifier"
  handler       = "bootstrap"
  runtime       = "provided.al2023"

  cloudwatch_logs_retention_in_days = 1
  trigger_on_package_timestamp      = false

  # https://github.com/terraform-aws-modules/terraform-aws-lambda/issues/36#issuecomment-650217274
  create_current_version_allowed_triggers = false

  environment_variables = {
    SSMSlackChannelIdArn = module.ssm_params[local.slack_channel_id_name].ssm_parameter_arn
    SSMSlackTokenArn     = module.ssm_params[local.slack_token_name].ssm_parameter_arn
  }

  source_path = [
    {
      path = "${path.module}/../src/friday_roadshow_notifier"
      commands = [
        "cargo lambda build --release",
        "cd target/lambda/friday_roadshow_notifier",
        ":zip",
      ]
      patterns = [
        "!.*",
        "bootstrap",
      ]
    }
  ]

  allowed_triggers = {
    DailyReminder = {
      principal  = "events.amazonaws.com"
      source_arn = module.event_bridge.eventbridge_rule_arns["friday_roadshow_notifier"]
    }
  }

  attach_policy_json = true
  policy_json = jsonencode({
    "Version" = "2012-10-17"
    "Statement" = [{
      "Effect" = "Allow"
      "Action" = [
        "ssm:GetParameter"
      ]
      "Resource" = [
        module.ssm_params[local.slack_channel_id_name].ssm_parameter_arn,
        module.ssm_params[local.slack_token_name].ssm_parameter_arn
      ]
    }]
  })

}
