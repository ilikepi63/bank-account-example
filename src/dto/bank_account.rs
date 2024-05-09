use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    // Account ID - use of Uuid as opposed to a long(u64). Usually long's infer an auto-incremental approach 
    // which is highly unlikely for an account based system - as the table would be easily partitionable via
    // the account_id. Use of Uuid was preferred for the above reason.
    pub account_id: Uuid, 
    pub amount: BigDecimal,
}