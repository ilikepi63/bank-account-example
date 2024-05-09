use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::repositories::BankAccount;


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequestDto {
    #[serde(flatten)]
    account: BankAccount
}

impl WithdrawalRequestDto {
    pub fn account_id(&self) -> &Uuid {
        &self.account.account_id
    }

    pub fn amount(&self) -> &BigDecimal {
        &self.account.amount
    }
}