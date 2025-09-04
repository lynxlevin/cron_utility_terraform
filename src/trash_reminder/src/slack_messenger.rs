use aws_config::BehaviorVersion;
use lambda_runtime::tracing;
use reqwest::Client;
use std::{collections::HashMap, time::Duration, env};

const SLACK_API_URL: &str = "https://slack.com/api/chat.postMessage";

pub struct SlackMessenger {
    ssm_client: aws_sdk_ssm::Client,
}

impl SlackMessenger {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        Self {
            ssm_client,
        }
    }

    pub async fn send_message(&self, text: String) -> Result<(), String> {
        let channel_id_ssm_arn = env::var("SSMSlackChannelIdArn").unwrap();
        let channel_id = self
            ._get_ssm_parameter(channel_id_ssm_arn)
            .await?;

        let token_ssm_arn = env::var("SSMSlackTokenArn").unwrap();
        let token = self
            ._get_ssm_parameter(token_ssm_arn)
            .await?;

        let mut body = HashMap::new();
        body.insert("channel", channel_id);
        body.insert("text", text);

        let client = Client::new();
        let res = client
            .post(SLACK_API_URL)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .timeout(Duration::from_secs(30))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Some error on requesting to Slack, error: {:?}", e);
                Err("Some error on requesting to Slack".to_string())
            }
        }
    }

    async fn _get_ssm_parameter(&self, arn: String) -> Result<String, String> {
        let res = match self
            .ssm_client
            .get_parameter()
            .name(&arn)
            .with_decryption(true)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                tracing::error!("Error on get_ssm_parameter, arn: {}, error: {:?}", arn, e);
                return Err("Some Error on get_ssm_parameter".to_string());
            }
        };

        match res.parameter() {
            Some(param) => match param.value() {
                Some(value) => Ok(value.to_string()),
                None => {
                    tracing::error!("Empty value, arn: {}", arn);
                    return Err("Some empty value".to_string());
                }
            },
            None => {
                tracing::error!("Empty value, arn: {}", arn);
                return Err("Some empty value".to_string());
            }
        }
    }
}
