pub use anchor_lang::prelude::*;
use anchor_spl::{self, Mint, Token, TokenAccount};

use crate::errors::ErrorCodes;
pub use crate::states::{ActiveLoan, CollectionPool, Offer};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(
        mut,
        seeds=[b"active-loan", offer.key().as_ref()],
        bump=active_loan.bump
    )]
    pub active_loan: Box<Account<'info, ActiveLoan>>,

    #[account(mut)]
    pub offer: Box<Account<'info, Offer>>,

    #[account(mut)]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = vault_asset_account.mint == asset_mint.key(),
        constraint = vault_asset_account.owner == vault_authority.key()
    )]
    pub vault_asset_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = lender_asset_account.mint == asset_mint.key(),
        constraint = lender_asset_account.owner == lender.key()
    )]
    pub lender_asset_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lender: Signer<'info>,

    pub vault_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<Liquidate>) -> Result<()> {
    let active_loan = &mut ctx.accounts.active_loan;
    let collection = &mut ctx.accounts.collection_pool;

    if active_loan.is_repaid {
        return Err(ErrorCodes::LoanAlreadyRepaid.into());
    }

    active_loan.is_liquidated = true;

    let (_vault_authority, vault_auth_bump) =
        Pubkey::find_program_address(&[collection.key().as_ref()], ctx.program_id);

    let cool_seeds = collection.key();

    let authority_seeds = &[col_seeds.as_ref(), &[vault_auth_bump]];

    let signer = &[&authority_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_asset_account.to_account_info().clone(),
        to: ctx.accounts.lender_asset_account.to_account_info().clone(),
        authority: ctx.accounts.vault_authority.clone(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info().clone(),
        cpi_accounts,
        signer,
    );

    token::transfer(cpi_ctx, 1)?;

    Ok(())
}

fn repayment_time_over<'info>(
    active_loan: &Account<'info, ActiveLoan>,
    clock: &Sysvar<'info, Clock>,
) -> Result<()> {
    if (!active_loan.repay_ts < clock.unix_timestamp) {
        return Err(ErrorCodes::CannotLiquidateYet.into());
    }

    Ok(())
}
