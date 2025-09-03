use aws_config::BehaviorVersion;
use lambda_runtime::tracing;
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

const SLACK_API_URL: &str = "https://slack.com/api/chat.postMessage";

pub struct SlackMessenger {
    config: aws_config::SdkConfig,
    ssm_client: aws_sdk_ssm::Client,
    ssm_parameter_arn: String,
}

impl SlackMessenger {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let ssm_client = aws_sdk_ssm::Client::new(&config);
        Self {
            config,
            ssm_client,
            // MYMEMO: get arn from env
            ssm_parameter_arn: "Some name from env".to_string(),
        }
    }

    pub async fn send_message(&self, text: String) -> Result<(), String> {
        // MYMEMO: change key.
        let channel = self
        .get_ssm_parameter(format!("{}/Channel", self.ssm_parameter_arn))
        .await?;
        // MYMEMO: change key.
        let token = self
            .get_ssm_parameter(format!("{}/Token", self.ssm_parameter_arn))
            .await?;

        let mut body = HashMap::new();
        body.insert("channel", channel);
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

    async fn get_ssm_parameter(& self, key: String) -> Result<String, String> {
        let res = match self
            .ssm_client
            .get_parameter()
            .name(self.ssm_parameter_arn.clone())
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                tracing::error!("Error on get_ssm_parameter, key: {}, error: {:?}", key, e);
                return Err("Some Error on get_ssm_parameter".to_string());
            }
        };

        match res.parameter() {
            Some(param) => match param.value() {
                Some(value) => Ok(value.to_string()),
                None => {
                    tracing::error!("Empty value, key: {}", key);
                    return Err("Some empty value".to_string());
                }
            },
            None => {
                tracing::error!("Empty value, key: {}", key);
                return Err("Some empty value".to_string());
            }
        }
    }
}
