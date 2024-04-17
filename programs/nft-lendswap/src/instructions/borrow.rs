pub use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::ErrorCodes;
use crate::states::{ActiveLoan, CollectionPool, Offer, Vault};

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(
        init,
        seeds = [b"active-loan", offer_loan.key().as_ref()],
        bump,
        payer = borrower,
        space = ActiveLoan::LEN,
    )]
    pub active_loan: Box<Account<'info, ActiveLoan>>,

    #[account(mut)]
    pub offer_loan: Box<Account<'info, Offer>>,

    #[account(mut)]
    pub vault_account: Box<Account<'info, Vault>>,

    #[account(
        init,
        seeds = [
            b"vault-asset-account",
            offer_loan.key().as_ref(),
        ],
        bump,
        payer = borrower,
        token::mint = asset_mint,
        token::authority = vault_authority
    )]
    pub vault_asset_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        constraint = borrower_asset_account.owner == *borrower.key,
        constraint = borrower_asset_account.mint == *asset_mint.to_account_info().key
    )]
    pub borrower_asset_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> Borrow<'info> {
    fn transfer_to_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.borrower_asset_account.to_account_info().clone(),
            to: self.vault_asset_account.to_account_info().clone(),
            authority: self.borrower.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
