use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use chrono::FixedOffset;
use lambda_runtime::{tracing, Error, LambdaEvent};

use crate::{slack_messenger::SlackMessenger, trash_schedule::{get_trash_schedule, Trash}};

const JST_OFFSET: i32 = 9;

pub(crate) async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    let utc_time = event.payload.time;
    let today = utc_time
        .with_timezone(&FixedOffset::east_opt(JST_OFFSET * 3600).unwrap())
        .date_naive();

    let trash = get_trash_schedule(today);

    match trash {
        Trash::None | Trash::Combustibles | Trash::Plastics => Ok(()),
        _ => {
            let message = format!("今日は{}の日です。", trash.to_string());

            let result = SlackMessenger::new().await.send_message(message).await;

            match result {
                Ok(_) => Ok(()),
                Err(e) => {
                    tracing::error!("{:?}", e);
                    Err(e.into())
                }
            }
        }
    }
}
