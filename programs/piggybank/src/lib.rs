use anchor_lang::prelude::*;
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token::{self, Token, TokenAccount, Mint};

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

    pub fn deposit_fund(ctx: Context<DepositFund>, amount: u64) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        let token_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.owner_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        bank.balance += amount;

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

    #[account(
        init, 
        payer = owner, 
        associated_token::mint = mint, 
        associated_token::authority = bank,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct DepositFund<'info> {
    #[account(mut)]
    pub bank: Account<'info, PiggyBank>,

    #[account(
        mut,
        associated_token::mint = bank.mint, 
        associated_token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = bank.mint, 
        associated_token::authority = bank,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

    #[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
    pub balance: u64,
}