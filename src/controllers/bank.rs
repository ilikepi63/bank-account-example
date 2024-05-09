use axum::{
    extract::State,
    routing::{post, MethodRouter},
    Json,
};

use crate::{
    dto::{WithdrawalEventDto, WithdrawalRequestDto, WithdrawalResponseDto},
    AppState,
};

pub struct BankAccountController;

impl BankAccountController {
    pub async fn withdraw(
        State(AppState {
            bank_account_repository,
            withdrawal_repository,
        }): State<AppState>,
        Json(payload): Json<WithdrawalRequestDto>,
    ) -> Json<WithdrawalResponseDto> {
        let withdrawal_result = bank_account_repository
            .withdrawal_from_account_id(payload.account_id(), payload.amount())
            .await;

        if withdrawal_result.is_ok() {
            withdrawal_repository.notify_withdrawal(WithdrawalEventDto::success(
                payload.account_id(),
                payload.amount(),
            ));
        }

        Json(WithdrawalResponseDto::success())
    }

    pub fn build() -> MethodRouter<AppState> {
        post(Self::withdraw)
    }
}
