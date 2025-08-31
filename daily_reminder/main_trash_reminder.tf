module "lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  function_name = "trash-reminder"
  description   = "My awesome lambda function"
  handler       = "trash_reminder.lambda_handler"
  runtime       = "python3.12"

  source_path = "../src"
}