use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::read_keypair_file,
        signer::Signer,
        transaction::Transaction,
    },
    Client, Cluster,
};
use solana_sdk::system_instruction;
use std::rc::Rc;
use std::time::Duration;
use std::thread;


//simply sends 100 trasactions to itself (also sends 100SOL to payer)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the client to connect to Surfpool's localnet
    let url = "http://localhost:8899";
    let payer = Rc::new(read_keypair_file("path/to/keypair")?);    
    let client = Client::new_with_options(
        Cluster::Custom(url.to_string(), url.to_string()),
        payer.clone(),
        CommitmentConfig::confirmed(),
    );

    // Get the program client for the system program
    let program = client.program(anchor_client::solana_sdk::system_program::ID)?;
    program.rpc().request_airdrop(&payer.pubkey(), 100_000_000_000)?; // 100 SOL
    // Send 100 transactions (1 lamport each to self)
    for i in 0..100 {
        // Create transfer instruction (1 lamport to self)
        let instruction = system_instruction::transfer(&payer.pubkey(), &payer.pubkey(), 1);

        // Build and send transaction - get fresh blockhash for each transaction
        let recent_blockhash = program.rpc().get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[payer.as_ref()],
            recent_blockhash,
        );

        match program.rpc().send_transaction(&tx) {
            Ok(signature) => println!("Transaction {} sent: {}", i + 1, signature),
            Err(e) => eprintln!("Transaction {} failed: {:?}", i + 1, e),
        }

        // Add longer delay to ensure transactions are processed
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
