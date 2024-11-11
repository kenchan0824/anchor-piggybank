use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("8izb58TQydRmNiWCygaQBrVeRgh1XUNPxZjkbWqr8dLj");

#[program]
pub mod piggy_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.owner.key();
        bank.mint = ctx.accounts.mint.key();
        bank.balance = 0;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
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
    
    pub fn close_bank(ctx: Context<CloseBank>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        
        // Transfer the balance to the owner
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: bank.to_account_info(),
        };

        let seeds = &[
            b"bank".as_ref(),
            bank.owner.as_ref(),
            bank.mint.as_ref(),
            &[ctx.bumps.bank]
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, bank.balance)?;

        // Close the vault account
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.owner_token_account.to_account_info(),
            authority: bank.to_account_info(),
        }; 
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::close_account(cpi_ctx)?;

        bank.balance = 0;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitBank<'info> {
    
    #[account(
        init,
        payer = owner,
        seeds = [b"bank", owner.key().as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + PiggyBank::INIT_SPACE,
    )]
    pub bank: Account<'info, PiggyBank>,

    #[account(
        init, 
        payer = owner, 
        seeds = [b"bank_vault", bank.key().as_ref()],
        bump,
        token::mint = mint, 
        token::authority = bank,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"bank", owner.key().as_ref(), bank.mint.as_ref()],
        bump,
        has_one = owner,
    )]
    pub bank: Account<'info, PiggyBank>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"bank_vault", bank.key().as_ref()],
        bump,
        token::mint = bank.mint, 
        token::authority = bank,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseBank<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"bank", owner.key().as_ref(), bank.mint.as_ref()],
        bump,
        has_one = owner,
    )]
    pub bank: Account<'info, PiggyBank>,

    #[account(
        mut,
        seeds = [b"bank_vault", bank.key().as_ref()],
        bump,
        token::mint = bank.mint, 
        token::authority = bank,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
}