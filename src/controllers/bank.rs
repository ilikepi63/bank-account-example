use axum::{
    extract::State,
    routing::{post, MethodRouter},
    Json,
};

use crate::{
    dto::{WithdrawalEventDto, WithdrawalRequestDto, WithdrawalResponseDto},
    AppState,

};
/// A [BankAccountController]
pub struct BankAccountController;

impl BankAccountController {
    /// Withdraw funds from a given account.
    pub async fn withdraw(
        // Extract the repositories
        State(AppState {
            bank_account_repository,
            withdrawal_repository,
        }): State<AppState>,
        // Extract the payload(body) - This was changed from the parameter based approach before. Not only will this hide
        // the person's account number from the request url in the HTTP request but is also more inline with RESTful principles.
        Json(payload): Json<WithdrawalRequestDto>,
    ) -> Json<WithdrawalResponseDto> {
        // Make the withdrawal query.
        let withdrawal_result = bank_account_repository
            .withdrawal_from_account_id(payload.account_id(), payload.amount())
            .await;


        // if the withdrawal was successful, add an event to the queue to process the message. 
        if withdrawal_result.is_ok() {
            withdrawal_repository.notify_withdrawal(WithdrawalEventDto::success(
                payload.account_id(),
                payload.amount(),
            ));
        }else{
            // In this instance, it might be beneficial to leave the error explicitly opaque.
            return Json(WithdrawalResponseDto::with_message("Failure to withdraw funds."))
        }

        Json(WithdrawalResponseDto::success())
    }

    pub fn build() -> MethodRouter<AppState> {
        post(Self::withdraw)
    }
}
