use anchor_lang::prelude::*;

use crate::states::CollectionPool;

#[derive(accounts)]
#[instruction(collection_id: Pubkey)]

pub struct CreatePool<'info> {
    #[account(
        init,
        seeds=[b"collection-pool", collection_id.key().as_ref()],
        bump,
        payer=authority,
        space=CollectionPool::LEN
    )]
    pub collection_pool: Box<Account<'info, CollectionPool>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreatePool>, collection_id: Pubkey, duration: i64) -> Result<()> {
    let collection = &mut ctx.accounts.collection_pool;

    collection.collection_id = collection_id;
    collection.pool_owner = ctx.accounts.authority.key();
    collection.duration = duration;
    collection.total_offers = 0;
    collection.bump = *ctx.bumps.get("collection_pool").unwrap();

    Ok(())
}
