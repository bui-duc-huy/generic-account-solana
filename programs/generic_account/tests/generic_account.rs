use anchor_lang::{system_program, ToAccountMetas};
use solana_program_test::*;
use anchor_lang::*;
use solana_sdk::signature::Keypair;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_program::{instruction::Instruction, sysvar::SysvarId};
use solana_sdk::transport::TransportError;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;


pub fn program_test() -> ProgramTest {
  let mut program = ProgramTest::new("generic_account", generic_account::id(), None);
  program
}

pub async fn airdrop(
  context: &mut ProgramTestContext,
  receiver: &Pubkey,
  amount: u64,
) -> Result<()> {
  let tx = Transaction::new_signed_with_payer(
    &[system_instruction::transfer(
      &context.payer.pubkey(),
      receiver,
      amount,
    )],
    Some(&context.payer.pubkey()),
    &[&context.payer],
    context.last_blockhash,
  );

  context.banks_client.process_transaction(tx).await.unwrap();
  Ok(())
}

pub async fn process_transaction(context: &mut ProgramTestContext, instructions: &Vec<Instruction>, signers: &Vec<&Keypair>) -> Result<()> {
  let tx = Transaction::new_signed_with_payer(
    instructions,
    Some(&signers[0].pubkey()),
    signers,
    context.last_blockhash,
  );

  context
    .banks_client
    .process_transaction(tx)
    .await
    .unwrap();

  Ok(())
}

#[tokio::test]
async fn generic_account() {
    let mut context = program_test().start_with_context().await; 

    let wallet = Keypair::new();
    airdrop(&mut context, &wallet.pubkey(), 1000000000000).await.unwrap();

    let data_info = Keypair::new();

    let create_type_1_data = generic_account::instruction::CreateType1 {
    }.data();

    let create_type_1_accounts = generic_account::accounts::CreateType1 {
        root: wallet.pubkey(),
        data_info: data_info.pubkey(),
        system_program: system_program::ID
    }.to_account_metas(None);

    let create_type_1_instruction = Instruction {
        accounts: create_type_1_accounts,
        data: create_type_1_data,
        program_id: generic_account::id()
    };

    let print_data = generic_account::instruction::PrintType {
    }.data();

    let print_data_account = generic_account::accounts::PrintType {
        root: wallet.pubkey(),
        data_info: data_info.pubkey(),
    }.to_account_metas(None);

    let print_data_instruction = Instruction {
        accounts: print_data_account,
        data: print_data,
        program_id: generic_account::id()
    };

    process_transaction(
        &mut context,
        &Vec::from([create_type_1_instruction, print_data_instruction]),
        &Vec::from([&wallet, &data_info]),
    )
    .await
    .unwrap();

}
