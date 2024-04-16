pub use anchor_lang::prelude::*;

use crate::states::{CollectionPool, Offer, Vault};

use crate::errors::ErrorCodes;

#[derive(Accounts)]
pub struct WithdrawOffer<'info> {
    #[account(
        mut,
        close = lender,
    )]
    pub offer_loan: Box<Account<'info, Offer>>,

    #[account(
        mut,
        close = lender
    )]
    pub vault_account: Account<'info, Vault>,

    #[account(mut)]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub lender: Signer<'info>,

    pub system_program: Program<'info, System>,
}
