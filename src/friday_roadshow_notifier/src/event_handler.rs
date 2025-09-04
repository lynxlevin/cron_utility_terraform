use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use chrono::{Datelike, Days, FixedOffset, Weekday};
use lambda_runtime::{tracing, Error, LambdaEvent};

use crate::{slack_messenger::SlackMessenger};

const JST_OFFSET: i32 = 9;

pub(crate) async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    let utc_time = event.payload.time;
    let today = utc_time
        .with_timezone(&FixedOffset::east_opt(JST_OFFSET * 3600).unwrap())
        .date_naive();
    let this_friday = today.checked_add_days(Days::new(Weekday::Fri.days_since(today.weekday()).into())).unwrap();

    let message = this_friday.format("https://kinro.ntv.co.jp/lineup/%Y%m%d").to_string();

    tracing::info!("today: {}", today);
    tracing::info!("friday: {}", this_friday);
    tracing::info!("message: {}", message);

    let result = SlackMessenger::new().await.send_message(message).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("{:?}", e);
            Err(e.into())
        }
    }
}
