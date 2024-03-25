use std::str::FromStr;

use solana_sdk::signature::Signer;
use solana_rpc_client::rpc_client;
use solana_sdk::signer::keypair;
use solana_sdk::transaction;
use solana_program::instruction;
use solana_program::pubkey;
use borsh::{BorshSerialize, BorshDeserialize};

const RPC_ADDR:&str= "https://api.devnet.solana.com";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub vault_bump_seed: u8,
    pub lamports: u64,
}

pub static VAULT_ACCOUNT_SIZE: u64 = 1024;

fn main() {
    let program_id = pubkey::Pubkey::from_str("CrMWLPYrya9Y99EUNsL1WpdEvZBDtFH3vUd7EmCux7ei").unwrap();

    let me = keypair::Keypair::from_base58_string("5P8C1ak7AGsGPHNYn5FTQPQFAFzY3bkBxWUcRN1G4pzPP1JF63yFsVqyvhJ45JNjNJsrHQXS5Pe676YKyGGHdwTw");
    println!("me is {}", me.pubkey());
    // let data_account = keypair::Keypair::new();

    let (expected_pda, bump_seed) = pubkey::Pubkey::find_program_address(&[b"vault"], &program_id);
    // let actual_pda = pubkey::Pubkey::create_program_address(&[b"vault", &[bump_seed]], &program_id).unwrap();

    // println!("expected_pda {} actual_pda {} bump_seed {}",expected_pda,  actual_pda, bump_seed);

    let client = rpc_client::RpcClient::new(RPC_ADDR);

    let account_metas = vec![
        instruction::AccountMeta::new(me.pubkey(), true),
        instruction::AccountMeta::new(expected_pda, false),
    ];

    let rent_lamports = client.get_minimum_balance_for_rent_exemption(VAULT_ACCOUNT_SIZE.try_into().unwrap()).unwrap();

    let instr = InstructionData{
        vault_bump_seed: bump_seed,
        lamports: rent_lamports,
    };

    println!("instr {:?}", instr);


    let mut writer = Vec::new();
    instr.serialize(&mut writer).unwrap();

    let instruction = instruction::Instruction::new_with_bytes(
        program_id, 
        &writer,
        account_metas,
    );

    println!("instruction {:?}", instruction);


    let ixs = vec![instruction];

    let latest_blockhash = client.get_latest_blockhash().unwrap();


    let sig = client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
        &ixs,
        Some(&me.pubkey()),
            &[&me],
            latest_blockhash,
        )).unwrap();

    println!("tx:{}", sig);
}
