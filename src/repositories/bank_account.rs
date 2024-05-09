use axum::extract::FromRef;
use bigdecimal::BigDecimal;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
use crate::{
    errors::{error, GenericResult},
    AppState,
};


/// The primary [BankAccountRepository] that allows us to abstract out 
/// integration to our underlying SQL database.
#[derive(Clone)]
pub struct BankAccountRepository {
    #[allow(unused)]
    database_connection: Pool<Postgres>,
}

/// Helper method to get this from AppState
impl FromRef<AppState> for BankAccountRepository {
    fn from_ref(app_state: &AppState) -> BankAccountRepository {
        app_state.bank_account_repository.clone()
    }
}

impl BankAccountRepository {
    /// Helper method to instantiate this from an Environment.
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

    /// Wrapper around withdrawing an amount from an account.
    pub async fn withdrawal_from_account_id(
        &self,
        account_id: &Uuid,
        amount: &BigDecimal,
    ) -> GenericResult<()> {
        // Update the bank_account. 
        // This is different from the original script as it removes the "read then write" consistency problem. 
        // Ideally we are using the transactional consistency of the database to ensure we don't get a race condition 
        // when checking the balance of a person's account.
        let result = sqlx::query!(
            "UPDATE bank_account SET balance = balance - $2 WHERE account_id = $1 AND balance >= $2;",
            account_id,
            amount
        )
        .execute(self.database_connection)
        .await?.rows_affected();

        let rows_affected = result.rows_affect();

        // This could ideally be improved by interpreting the result from sqlx above. I chose not to do this
        // as the only other errors I could really think of(database error, no account found) should likely be 
        // kept opaque from the user. 
        match rows_affected {
            1 => Ok(()),
            _ => {
                let message = format!("Failed to update the Bank Account: {:#?}", result);
                error!("{message}");

                error(&message)
            },
        }
    }
}
