//! [Banking] API
#![deny(missing_docs)]

#[macro_use]
extern crate log;

use axum::Router;
use errors::GenericResult;
use repositories::{BankAccountRepository, WithdrawalRepository};

mod controllers;
mod dto;
mod errors;
mod repositories;


/// The [AppState]'s primary responsibility to is to serve as axum's app state to hold specific data. 
/// This works similarly but not quite like Sping's dependency injection framework.
#[derive(Clone)]
struct AppState {
    bank_account_repository: BankAccountRepository,
    withdrawal_repository: WithdrawalRepository,
}

impl AppState {
    /// Helper method for creating an instance of this struct from the environment.
    pub async fn from_env() -> GenericResult<Self> {
        let bank_account_repository = BankAccountRepository::from_env().await?;
        let withdrawal_repository = WithdrawalRepository::from_env().await?;

        Ok(Self {
            bank_account_repository,
            withdrawal_repository,
        })
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    // Initialize tracing - helps quite a lot with debugging tokio tasks.
    tracing_subscriber::fmt::init();

    // Retrieve the PORT from the environment.
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    // Build our application with our controllers.
    let app = Router::new()
        .route("/bank", controllers::BankAccountController::build())
        .with_state(AppState::from_env().await?);

    // Run the app
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
