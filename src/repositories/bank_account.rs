use axum::extract::FromRef;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::{
    errors::{error, GenericResult},
    AppState,
};

#[derive(Serialize, Deserialize)]
pub struct BankAccount {
    pub account_id: Uuid,
    pub amount: BigDecimal,
}

#[derive(Clone)]
pub struct BankAccountRepository {
    #[allow(unused)]
    database_connection: Pool<Postgres>,
}

impl FromRef<AppState> for BankAccountRepository {
    fn from_ref(app_state: &AppState) -> BankAccountRepository {
        app_state.bank_account_repository.clone()
    }
}

impl BankAccountRepository {
    pub async fn from_env() -> GenericResult<Self> {
        let db_connection_str = std::env::var("DATABASE_URL")?;

        // set up connection pool
        let pool = PgPoolOptions::new()
            .connect(&db_connection_str)
            .await
            .expect("can't connect to database");

        Ok(Self {
            database_connection: pool,
        })
    }

    pub async fn withdrawal_from_account_id(
        &self,
        _account_id: &Uuid,
        _amount: &BigDecimal,
    ) -> GenericResult<()> {
        let rows_affected = 1;
        // let rows_affected = sqlx::query!(
        //     "UPDATE bank_account SET balance = balance - $2 WHERE account_id = $1 AND balance >= $2;",
        //     account_id,
        //     amount
        // )
        // .execute(self.database_connection)
        // .await?.rows_affected();

        match rows_affected {
            1 => Ok(()),
            _ => error("Failed to update the Bank Account"),
        }
    }
}
