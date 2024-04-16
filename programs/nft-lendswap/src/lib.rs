use anchor_lang::prelude::*;

declare_id!("E4AH1YiTvtnHbqAAX64Sx7sFq892vE64LzFnZYd8yjLE");

#[program]
pub mod nft_lendswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
