pub use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::token::{ self, Mint, Token, TokenAccount, Transfer };

pub use crate::states::{ ActiveLoan, CollectionPool, Offer, Vault };

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub active_loan: Box<Account<'info, ActiveLoan>>,

    #[account(mut)]
    pub offer: Box<Account<'info, Offer>>,

    #[account(mut)]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(
        mut, 
        constraint = lender.key() == offer.lender.key()
    )]
    pub lender: AccountInfo<'info>,

    #[account(mut)]
    pub asset_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = borrower_asset_account.mint == asset_mint.key(),
        constraint = vault_asset_account.owner == vault_authority.key()
    )]

    pub vault_asset_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_account: Account<'info, Vault>,

    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub borrower: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

impl<'info> Repay<'info> {
    fn transfer_to_lender_context(&self, ) -> CpiContext<'_, '_, '_, 'info, system_program::Transfer<'info>> {
        let cpi_accounts = system_program::Transfer {
            from: self.borrower.to_account_info().clone(),
            to: self.lender.clone(),
        };

        CpiContext::new(self.system_program.to_account_info().clone(), cpi_accounts)
    }
}

pub fn handler(ctx: Context<Repay>) -> Result<()> {
    
    let active_loan = &mut ctx.accounts.active_loan;
    let collection = &mut ctx.accounts.collection_pool;
    let offer = &mut ctx.accounts.offer;

    active_loan.is_repaid = true;

    let (_vault_authority, vault_auth_bump) = 
        Pubkey::find_program_address(&[collection.key().as_ref()], ctx.program_id);

    let col_seeds = collection.key();

    let authority_seeds = &[col_seeds.as_ref(), &[vault_auth_bump]];

    let signer = &[&authority_seeds[..]];

    let repay_amount = offer.repay_lamport_amount;

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_asset_account.to_account_info().clone(),
        to: ctx
            .accounts
            .borrower_asset_account
            .to_account_info
            .clone(),
        authority: ctx.accounts.vault_authority.clone(),    
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info().clone(),
        cpi_accounts,
        signer
    );

    token::transfer(cpi_ctx, 1)?;

    system_program::transfer(ctx.accounts.transfer_to_lender_context(), repay_amount)?;
    
    Ok(())
}