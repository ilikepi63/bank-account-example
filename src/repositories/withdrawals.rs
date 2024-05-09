use std::time::Duration;

use axum::extract::FromRef;
use crate::{dto::WithdrawalEventDto, errors::GenericResult, AppState};

/// This constant was moved out from some generic code, but can be in turn moved into an
/// environment variable if need be within the `from_env` function below.
pub static WITHDRAWAL_EVENT_TOPIC_ARN: &str = "withdrawal_events";

/// Our primary [WithdrawalRepository]. This was fundamentally hard to name, but I surmise that "withdrawal"
/// is appropriate as that could be considered an event and there could be modeled after the face.
#[derive(Clone)]
pub struct WithdrawalRepository {
    client: aws_sdk_sns::Client,
}

impl FromRef<AppState> for WithdrawalRepository {
    /// Helper method to return this repository from AppState
    fn from_ref(app_state: &AppState) -> WithdrawalRepository {
        app_state.withdrawal_repository.clone()
    }
}

static RETRY_TIMES: u8 = 5;

impl WithdrawalRepository {
    /// Helper method to instantiate this struct from an environment.
    pub async fn from_env() -> GenericResult<Self> {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_sns::Client::new(&config);

        Ok(Self { client })
    }

    /// Submit a withdrawal event to an SNS queue.
    pub fn notify_withdrawal(&self, event: WithdrawalEventDto) {
        // we clone this client - this is just a reference clone: https://docs.rs/aws-sdk-sns/latest/src/aws_sdk_sns/client.rs.html#79
        // so we're not continuously copying the client - which could become expensive.
        let client = self.client.clone();

        // We move this operation to a tokio task - this doesn't spawn a new thread but a lightweight task similar in concept to
        // a goroutine - https://tokio.rs/tokio/tutorial/spawning https://go.dev/tour/concurrency/1
        tokio::spawn(async move {
            // Serialize the event to JSON
            if let Result::Ok(message) = serde_json::to_string(&event) {
                // Retry adding this message on a failure using an exponential backoff algorithm.
                for i in 0..=RETRY_TIMES {
                    // publish the event
                    let result = client
                        .publish()
                        .topic_arn(WITHDRAWAL_EVENT_TOPIC_ARN)
                        .message(message)
                        .send()
                        .await;

                    match result {
                        Ok(_) => break,
                        Err(e) => {
                            // sleep this tokio thread - this does not block but schedules a wake later.
                            tokio::time::sleep(Duration::from_secs(2_u64.pow(i.into())));
                            error!("Failed to publish to SNS: {:#?}", e)
                        }
                    }
                }
            }
        });
    }
}
