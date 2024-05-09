use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponseDto {
    // Ideally this would have it's own error code with a enumeration 
    // implementing the different errors.
    message: String,
}

impl WithdrawalResponseDto {
    pub fn success() -> Self {
        Self {
            message: "Success".to_string(),
        }
    }

    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
