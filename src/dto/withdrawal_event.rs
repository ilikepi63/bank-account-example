use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WithdrawalStatus {
    Success,
    Failure,
}

#[derive(Serialize)]
pub struct WithdrawalEventDto {
    account_id: Uuid,
    amount: BigDecimal,
    status: WithdrawalStatus,
}

impl WithdrawalEventDto {
    pub fn success(account_id: &Uuid, amount: &BigDecimal) -> Self {
        Self {
            account_id: account_id.clone(),
            amount: amount.clone(),
            status: WithdrawalStatus::Success,
        }
    }
}
