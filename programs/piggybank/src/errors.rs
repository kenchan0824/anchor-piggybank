use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Bank not ready to be closed")]
    BankNotReady,
}