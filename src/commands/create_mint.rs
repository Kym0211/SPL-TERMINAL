use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    signature::{read_keypair_file, Keypair}, 
    signer::Signer, 
    system_instruction::create_account, 
    transaction::Transaction
};
use spl_token::{
    instruction::initialize_mint,
    solana_program::program_pack::Pack, 
    state::Mint
};
use anyhow::{anyhow, Result};


pub fn create_mint(rpc_url: &String, keypair: &String) -> Result<()>{

    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer = read_keypair_file(keypair).expect("Failed to read keypair");

    let rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

    let mint_account = Keypair::new();

    let create_account_ix = create_account(
        &payer.pubkey(), 
        &mint_account.pubkey(), 
        rent, 
        Mint::LEN as u64, 
        &spl_token::id()
    );

    let initialize_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint_account.pubkey(),
        &payer.pubkey(),
        None,
        6
    )?;

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Blockhash error: {}", e))?;

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix], 
        Some(&payer.pubkey()), 
        &[&payer, & &mint_account], 
        recent_blockhash
    );

    let signature = client
        .send_and_confirm_transaction(&tx)
        .map_err(|e| anyhow!("Transaction failed: {}", e))?;

    println!("✅ Mint created: {}", mint_account.pubkey());
    println!("📝 Transaction: https://solscan.io/tx/{}?cluster=devnet", signature);

    Ok(())
}