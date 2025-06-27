use std::{str::FromStr, vec};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, program_pack::Pack, pubkey::Pubkey, signature::read_keypair_file, signer::Signer, transaction::Transaction
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account
};
use spl_token::{
    instruction::transfer,
    state::Account
};
use anyhow::{anyhow, Result};

pub fn transfer_tokens(
    rpc_url: &String, 
    keypair: &String, 
    destination: &String, 
    mint: &String,
    amount: u64
) -> Result<()>{

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = read_keypair_file(keypair).expect("Failed to read keypair");
    
    let source_pubkey = payer.pubkey();
    let destination_pubkey = Pubkey::from_str(destination)?;
    let mint_pubkey = Pubkey::from_str(mint)?;

    let source_ata = get_associated_token_address(&source_pubkey, &mint_pubkey);

    let source_account = client.get_account(&source_ata)?;
    let source_token_data = Account::unpack(&source_account.data)?;

    if source_token_data.amount < amount {
        return Err(anyhow!(
            "Insufficient balance! Available: {}, Required: {}",
            source_token_data.amount,
            amount
        ));
    }

    let destination_ata = get_associated_token_address(&destination_pubkey, &mint_pubkey);

    let mut instructions: Vec<Instruction> = vec![];

    
    if client.get_account(&destination_ata).is_err() {
        let create_ix = create_associated_token_account(
            &payer.pubkey(), 
            &destination_pubkey, 
            &mint_pubkey, 
            &spl_token::id()
        );
        instructions.push(create_ix);
    }
    
    let transfer_ix = transfer(
        &spl_token::id(), 
        &source_ata, 
        &destination_ata, 
        &payer.pubkey(), 
        &[], 
        amount
    )?;
    
    instructions.push(transfer_ix);
    
    let recent_blockhash = client
    .get_latest_blockhash()
    .map_err(|e| anyhow!("Blockhash error: {}", e))?;

    let tx = Transaction::new_signed_with_payer(
        &instructions, 
        Some(&payer.pubkey()), 
        &[&payer], 
        recent_blockhash
    );

    println!("Here........");

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| anyhow!("Transaction failed: {}", e))?;

    println!("✅ Transferred {} tokens", amount);
    println!("📝 Transaction: https://solscan.io/tx/{}?cluster=devnet", signature);

    Ok(())
}
