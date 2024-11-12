# Solana Piggy Bank Program with Anchor

This is a Solana program written using the Anchor framework that allows users to save and get back their tokens from a piggy bank. 

Further to the previous Counter program which demonstrate how to store and manipulate data on Solana, this program demonstrates:

- Transfer of tokens
- Custody of user's tokens to a vault owned by PDA
- Closure of normal accounts and token accounts
- Using clock Solana
- Handling predefined errors

## Features

### Initialization

The PiggyBank program initializes a bank account with specifying the owner, the token mint allowed and the lock period.

```rust
pub fn init_bank(ctx: Context<InitBank>, timeout: i64)
```

### Deposit Tokens

Deposit tokens into the bank.

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64)
```

### Close the Bank

Check if the lock period expiries, withdraw all tokens deposited and close the bank.

```rust
pub fn close_bank(ctx: Context<CloseBank>)
```

### Accounts

The program uses the following account structures:

```rust
#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub start_time: i64,
    pub timeout: i64,
}
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)



## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/kenchan0824/anchor-piggybank.git
cd solana-piggybank
```

### 2. Run the Test

```sh
anchor test
```

## Tests
The tests for the counter program are located in

piggybank.ts

The tests include:

- Initialize the piggy bank
- Deposit into piggy bank
- Close the piggy bank should fail before timeout
- Close the piggy bank should fail before timeout
- Close the piggy bank

## License

This project is licensed under the MIT License. See the LICENSE file for details.