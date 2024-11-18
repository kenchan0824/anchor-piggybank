use anchor_lang::prelude::*;
use anchor_spl::token;

pub mod states;
pub mod contexts;
pub mod errors;

use contexts::*;
use errors::ErrorCode;

declare_id!("8izb58TQydRmNiWCygaQBrVeRgh1XUNPxZjkbWqr8dLj");

#[program]
pub mod piggy_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>, timeout: i64) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.owner = ctx.accounts.owner.key();
        bank.mint = ctx.accounts.mint.key();
        bank.balance = 0;
        bank.start_time = Clock::get()?.unix_timestamp;
        bank.timeout = timeout;

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
        
        // Check if the bank is ready to be closed
        let current_time = Clock::get()?.unix_timestamp;
        if current_time < bank.start_time + bank.timeout {
            return Err(ErrorCode::BankNotReady.into());
        }
        
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
