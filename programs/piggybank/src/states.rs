use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub start_time: i64,
    pub timeout: i64,
}
