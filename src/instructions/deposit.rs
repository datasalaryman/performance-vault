use quasar_lang::prelude::*;

#[derive(Seeds)]
#[seeds(b"vault", user: Address)]
pub struct VaultPda;

#[derive(Accounts)]
#[instruction(_amount: u64, vault_bump: u8)]
pub struct Deposit {
    #[account(mut)]
    pub user: Signer,
    #[account(
        mut, 
        address = VaultPda::seeds(user.address()).with_bump(vault_bump)
    )]
    pub vault: UncheckedAccount,
    pub system_program: Program<SystemProgram>,

}

impl Deposit {
    #[inline(always)]
    pub fn deposit(&self, amount: u64) -> Result<(), ProgramError> {
        self.system_program
            .transfer(&self.user, &self.vault, amount)
            .invoke()
    }
}
