module "event_bridge" {
  source = "terraform-aws-modules/eventbridge/aws"

  create_bus = false

  rules = {
    trash_reminder = {
      schedule_expression = "rate(1 minute)"
      # schedule_expression = "cron(30 21 * * ? *)"
    }
  }

  targets = {
    trash_reminder = [
      {
        name = "trash-reminder-lambda"
        arn = module.lambda_function.lambda_function_arn
      }
    ]
  }
}

module "lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "trash-reminder"
  description   = "My awesome lambda function"
  handler       = "bootstrap"
  runtime       = "provided.al2023"

  cloudwatch_logs_retention_in_days = 1
  trigger_on_package_timestamp = false

  # https://github.com/terraform-aws-modules/terraform-aws-lambda/issues/36#issuecomment-650217274
  create_current_version_allowed_triggers = false

  source_path = [
    {
      path = "${path.module}/../src/trash_reminder"
      commands = [
        "cargo lambda build --release",
        "cd target/lambda/trash_reminder",
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
      principal = "events.amazonaws.com"
      source_arn = module.event_bridge.eventbridge_rule_arns["trash_reminder"]
    }
  }
}