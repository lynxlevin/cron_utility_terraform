terraform {
  backend "s3" {
    bucket = "lynxlevin-tfstate"
    key    = "cron_utility/daily_reminder"
    region = "ap-northeast-1"
  }
}
