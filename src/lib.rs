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
        // deposit logic
        ctx.accounts.deposit(amount);
        Ok(())
    }

    #[instruction(discriminator = 1)]
    pub fn withdraw(ctx: Ctx<Withdraw>) -> Result<(), ProgramError> {
        // withdraw logic
        ctx.accounts.withdraw();
        Ok(())
    }
}

#[cfg(test)]
mod tests;
