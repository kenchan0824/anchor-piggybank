use anchor_lang::prelude::*;

declare_id!("8izb58TQydRmNiWCygaQBrVeRgh1XUNPxZjkbWqr8dLj");

#[program]
pub mod piggy_bank {
    use super::*;

    pub fn open_bank(ctx: Context<OpenBank>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.owner.key();
        bank.balance = 0;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct OpenBank<'info> {
    
    #[account(
        init,
        seeds = ["bank".as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + PiggyBank::INIT_SPACE,
        payer = owner
    )]
    pub bank: Account<'info, PiggyBank>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub bump: u8,
    pub balance: u64,
}