use quasar_lang::prelude::*;
use super::deposit::VaultPda;

#[derive(Accounts)]
#[instruction(vault_bump: u8)]
pub struct Withdraw {
    #[account(mut)]
    pub user: Signer,
    #[account(
        mut, 
        address = VaultPda::seeds(user.address()).with_bump(vault_bump)
    )]
    pub vault: UncheckedAccount,
    pub system_program: Program<SystemProgram>,

}

impl Withdraw {
    #[inline(always)]
    pub fn withdraw(&self, vault_bump: u8) -> Result<(), ProgramError> {
        let amount = self.vault.to_account_view().lamports();

        quasar_lang::address::AddressVerify::with_signer_seeds(
            &VaultPda::seeds(self.user.address()).with_bump(vault_bump),
            &[],
            |signers| {
                self.system_program
                    .transfer(&self.vault, &self.user, amount)
                    .invoke_with_signers(signers)
            },
        )
    }
}
