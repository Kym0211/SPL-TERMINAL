use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey, signature::read_keypair_file, signer::Signer, transaction::Transaction
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    instruction::burn,
    state::Account
};
use anyhow::{anyhow, Result};

pub fn burn_tokens(
    rpc_url: &String, 
    keypair: &String,
    mint: &String,
    amount: u64
) -> Result<()>{

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = read_keypair_file(keypair).expect("Failed to read keypair");
    let mint_pubkey = Pubkey::from_str(mint)?;

    let source_ata = get_associated_token_address(&payer.pubkey(), &mint_pubkey);

    let source_account = client.get_account(&source_ata)?;
    let source_token_data = Account::unpack(&source_account.data)?;

    if source_token_data.amount < amount {
        return Err(anyhow!(
            "Insufficient balance! Available: {}, Required: {}",
            source_token_data.amount,
            amount
        ));
    }

    let burn_ix = burn(
        &spl_token::id(), 
        &source_ata, 
        &mint_pubkey, 
        &payer.pubkey(), 
        &[], 
        amount
    )?;
    
    let recent_blockhash = client
    .get_latest_blockhash()
    .map_err(|e| anyhow!("Blockhash error: {}", e))?;

    let tx = Transaction::new_signed_with_payer(
        &[burn_ix], 
        Some(&payer.pubkey()), 
        &[&payer], 
        recent_blockhash
    );

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| anyhow!("Transaction failed: {}", e))?;

    println!("✅ Transferred {} tokens", amount);
    println!("📝 Transaction: https://solscan.io/tx/{}?cluster=devnet", signature);

    Ok(())
}
