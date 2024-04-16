use anchor_lang::prelude::*;

pub struct Offer {
    // Collection
    pub collection: Pubkey,
    // Offer Amount
    pub offer_lamport_amount: u64,
    // Repay Amount
    pub repay_lamport_amount: u64,
    // Lender
    pub lender: Pubkey,
    // Loan Taken
    pub is_loan_taken: bool,
    // Borrower
    pub borrower: Pubkey,
    // Bump
    pub bump: u8,
}

impl Offer {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 32 + 1 + 32 + 1;
}
