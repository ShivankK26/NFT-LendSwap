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

pub fn handler(ctx: Context<WithdrawOffer>, minimum_balance_for_rent_exemption: u64) -> Result<()> {
    let collection = &mut ctx.accounts.collection_pool;

    if ctx.accounts.offer_loan.is_loan_taken == true {
        return Err(ErrorCodes::LoanAlreadyTaken.into());
    }

    collection.total_offers -= 1;

    let vault_lamports_initial: u64 = ctx.accounts.vault_account.to_account_info().lamports();

    let transfer_amount = vault_lamports_initial
        .checked_sub(minimum_balance_for_rent_exemption)
        .unwrap();

    **ctx
        .accounts
        .vault_account
        .to_account_info()
        .try_borrow_mut_lamports()? -= transfer_amount;

    let mut lamports_ref = ctx.accounts.lender.try_borrow_mut_lamports()?;
    **lamports_ref += transfer_amount;

    Ok(())
}
