#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod instructions;
use instructions::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
mod performance_vault {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn deposit(ctx: Ctx<Deposit>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.deposit(amount)
    }

    #[instruction(discriminator = 1)]
    pub fn withdraw(ctx: Ctx<Withdraw>, vault_bump: u8) -> Result<(), ProgramError> {
        ctx.accounts.withdraw(vault_bump)
    }
}

#[cfg(test)]
mod tests;
