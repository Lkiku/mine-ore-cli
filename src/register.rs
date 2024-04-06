use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signer};
use tokio::time::{sleep, Duration};

use crate::{utils::proof_pubkey, Miner};

impl Miner {
    pub async fn register(&self) {
        // Return early if miner is already registered
        let signer = self.signer();
        let proof_address = proof_pubkey(signer.pubkey());
        let client =
            RpcClient::new_with_commitment(self.cluster.clone(), CommitmentConfig::confirmed());
        if client.get_account(&proof_address).await.is_ok() {
            return;
        }

        // Sign and send transaction with retry mechanism.
        println!("Generating challenge...");
        let ix = ore::instruction::register(signer.pubkey());
        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 10; // Maximum number of attempts before giving up
        
        loop {
            let cloned_ix = ix.clone();
            match self.send_and_confirm(&[cloned_ix]).await {
                Ok(_) => {
                    println!("Transaction confirmed");
                    break;
                }
                Err(e) => {
                    attempts += 1;
                    println!("Attempt {} failed: {:?}", attempts, e);
                    if attempts >= MAX_ATTEMPTS {
                        panic!("Transaction failed after {} attempts: {:?}", MAX_ATTEMPTS, e);
                    }
                    // Exponential backoff or fixed delay could be considered here
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}
