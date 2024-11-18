use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};

use crate::states::*;

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

