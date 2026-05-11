use quasar_lang::prelude::*;
use super::deposit::VaultPda;

#[derive(Accounts)]
pub struct Withdraw {
    #[account(mut)]
    pub user: Signer,
    #[account(
        mut, 
        address = VaultPda::seeds(user.address())
    )]
    pub vault: UncheckedAccount,
    pub system_program: Program<SystemProgram>,

}

impl Withdraw {
    #[inline(always)]
    pub fn withdraw(&self) -> Result<(), ProgramError> {
        let seeds = [
            Seed::from(b"vault" as &[u8]), 
            Seed::from(self.user.address().as_ref()), 
        ];

        let amount = self.vault.to_account_view().lamports();
        
        self.system_program
            .transfer(&self.vault, &self.user, amount)
            .invoke_signed(&seeds)
    }
}
