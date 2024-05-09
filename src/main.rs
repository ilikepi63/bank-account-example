//! [Banking] API
#![deny(missing_docs)]

#[macro_use]
extern crate log;


use std::time::Duration;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use errors::GenericResult;
use repositories::{BankAccountRepository, WithdrawalRepository};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

mod controllers;
mod dto;
mod errors;
mod repositories;

#[derive(Clone)]
struct AppState {
    bank_account_repository: BankAccountRepository,
    withdrawal_repository: WithdrawalRepository
}

impl AppState {
    pub async fn from_env() -> GenericResult<Self> {
        let bank_account_repository = BankAccountRepository::from_env().await?;
        let withdrawal_repository = WithdrawalRepository::from_env().await?;
        
        Ok(Self{
            bank_account_repository, withdrawal_repository
        })
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Environment Setup
    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    // build our application with our controllers
    let app = Router::new()
        .route("/bank", controllers::BankAccountController::build())
        .with_state(AppState::from_env().await?);

    // Run the app
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
