use anchor_lang::prelude::*;

declare_id!("8izb58TQydRmNiWCygaQBrVeRgh1XUNPxZjkbWqr8dLj");

#[program]
pub mod piggybank {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
