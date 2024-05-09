use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalResponseDto {
    message: String,
}

impl WithdrawalResponseDto {
    pub fn success() -> Self {
        Self {
            message: "Success".to_string(),
        }
    }
}
