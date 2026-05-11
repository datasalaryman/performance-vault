use quasar_svm::{Account, Pubkey, QuasarSvm};
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use solana_system_program::id as system_program; 

fn setup() -> QuasarSvm {
    let elf = std::fs::read("target/deploy/performance_vault.so").unwrap();
    QuasarSvm::new()
        .with_program(&Pubkey::from(crate::ID), &elf)
}

fn deposit_instruction (
    user: Address, 
    amount: u64
) -> Instruction {

    let (vault, vault_bump) = Pubkey::find_program_address(&[b"vault", user.as_ref()], &crate::ID);

    Instruction { 
        program_id: crate::ID, 
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(vault, true),
            AccountMeta::new_readonly(system_program(), false),
        ], 
        data: [vec![0], amount.to_le_bytes().to_vec(), vec![vault_bump]].concat()
    }
}

fn withdraw_instruction (
    user: Address, 
) -> Instruction {

    let (vault, vault_bump) = Pubkey::find_program_address(&[b"vault", user.as_ref()], &crate::ID);

    Instruction { 
        program_id: crate::ID, 
        accounts: vec![
            AccountMeta::new(user, true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program(), false),
        ], 
        data: vec![1, vault_bump]
    }
}


#[test]
fn test_deposit_withdraw_workflow() {
    let mut svm = setup();

    let payer = Pubkey::new_unique();

    let deposit = deposit_instruction(
        Address::from(payer.to_bytes()),
        10, 
    );

    let withdraw = withdraw_instruction(
        Address::from(payer.to_bytes()),
    );

    let result = svm.process_instruction_chain(
        &[deposit, withdraw],
        &[Account {
            address: payer,
            lamports: 10_000_000_000,
            data: vec![],
            owner: quasar_svm::system_program::ID,
            executable: false,
        }],
    );

    result.assert_success();
}
