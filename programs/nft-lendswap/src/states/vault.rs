use anchor_lang::prelude::*;

pub struct Vault {
    // Offer goes into the vault
    pub offer: Pubkey,
    // Bump
    pub bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 1;
}
