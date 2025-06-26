use std::{str::FromStr, vec};

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::Instruction, pubkey::Pubkey, signature::read_keypair_file, signer::Signer, transaction::Transaction
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account
};
use spl_token::instruction::mint_to;

use anyhow::{anyhow, Result};


pub fn mint_tokens(
    rpc_url: &String, 
    keypair: &String, 
    mint: &String, 
    recipient: &String, 
    amount: u64
) -> Result<()>{

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = read_keypair_file(keypair).expect("Failed to read keypair");
    
    let mint_pubkey = Pubkey::from_str(mint)?;
    let recipient_pubkey = Pubkey::from_str(recipient)?;

    let recipient_ata = get_associated_token_address(&recipient_pubkey, &mint_pubkey);

    let mut instructions: Vec<Instruction> = vec![];

    if client.get_account(&recipient_ata).is_err() {
        let create_ix = create_associated_token_account(
            &payer.pubkey(), 
            &recipient_pubkey, 
            &mint_pubkey, 
            &spl_token::id()
        );
        instructions.push(create_ix);
    }

    let mint_ix = mint_to(
        &spl_token::id(), 
        &mint_pubkey, 
        &recipient_ata, 
        &payer.pubkey(), 
        &[], 
        amount
    )?;

    instructions.push(mint_ix);

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Blockhash error: {}", e))?;

    let tx = Transaction::new_signed_with_payer(
        &instructions, 
        Some(&payer.pubkey()), 
        &[&payer], 
        recent_blockhash
    );

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| anyhow!("Transaction failed: {}", e))?;

    println!("✅ Minted {} tokens to {}", amount, recipient);
    println!("📝 Transaction: https://solscan.io/tx/{}?cluster=devnet", signature);

    Ok(())
}