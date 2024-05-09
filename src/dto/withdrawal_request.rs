use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

use super::BankAccount;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequestDto {
    #[serde(flatten)] // this flattens this struct into the nested struct. This is primarily for a better DRY effect.
    account: BankAccount,
}

impl WithdrawalRequestDto {
    pub fn account_id(&self) -> &Uuid {
        &self.account.account_id
    }

    pub fn amount(&self) -> &BigDecimal {
        &self.account.amount
    }
}
