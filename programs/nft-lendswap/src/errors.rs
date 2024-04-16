pub use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("Loan already taken.")]
    LoanAlreadyTaken,
    #[msg("Loan already repaid.")]
    LoanAlreadyRepaid,
    #[msg("Cannot liquidate loan yet.")]
    CannotLiquidateYet,
}
