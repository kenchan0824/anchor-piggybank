use anchor_lang::prelude::*;
use anchor_spl::token;

declare_id!("8izb58TQydRmNiWCygaQBrVeRgh1XUNPxZjkbWqr8dLj");

#[program]
pub mod piggy_bank {
    use super::*;

    pub fn open_bank(ctx: Context<OpenBank>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.owner.key();
        bank.mint = ctx.accounts.mint.key();
        bank.bump = ctx.bumps.bank;
        bank.balance = 0;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct OpenBank<'info> {
    
    #[account(
        init,
        seeds = ["bank".as_bytes(), owner.key().as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + PiggyBank::INIT_SPACE,
        payer = owner
    )]
    pub bank: Account<'info, PiggyBank>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, token::Mint>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
    pub balance: u64,
}