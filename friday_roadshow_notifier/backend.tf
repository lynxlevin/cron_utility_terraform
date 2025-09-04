terraform {
  backend "s3" {
    bucket = "lynxlevin-tfstate"
    key    = "cron_utility/friday_roadshow_notifier"
    region = "ap-northeast-1"
  }
}
