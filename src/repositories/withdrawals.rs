use anyhow::Ok;
use axum::extract::FromRef;

use crate::{dto::WithdrawalEventDto, errors::GenericResult, AppState};

pub static WITHDRAWAL_EVENT_TOPIC_ARN: &str = "withdrawal_events";

#[derive(Clone)]
pub struct WithdrawalRepository {
    client: aws_sdk_sns::Client,
}

impl FromRef<AppState> for WithdrawalRepository {
    fn from_ref(app_state: &AppState) -> WithdrawalRepository {
        app_state.withdrawal_repository.clone()
    }
}

impl WithdrawalRepository {
    pub async fn from_env() -> GenericResult<Self> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_sns::Client::new(&config);

        Ok(Self { client })
    }

    pub fn notify_withdrawal(&self, event: WithdrawalEventDto) {
        let client = self.client.clone();

        tokio::spawn(async move {
            if let Result::Ok(message) = serde_json::to_string(&event) {
                let result = client
                    .publish()
                    .topic_arn(WITHDRAWAL_EVENT_TOPIC_ARN)
                    .message(message)
                    .send()
                    .await;

                if let Err(e) = result {
                    error!("Failed to publish to SNS: {:#?}", e);
                }
            }
        });
    }
}
