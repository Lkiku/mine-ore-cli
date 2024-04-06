use std::time::Duration;

use cached::proc_macro::cached;
use ore::{
    self,
    state::{Proof, Treasury},
    utils::AccountDeserialize,
    MINT_ADDRESS, PROOF, TREASURY_ADDRESS,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, sysvar};
use solana_sdk::{clock::Clock, commitment_config::CommitmentConfig};
use spl_associated_token_account::get_associated_token_address;
use tokio::time::sleep;

pub async fn get_treasury(cluster: String) -> Treasury {
    let client = RpcClient::new_with_commitment(cluster, CommitmentConfig::confirmed());
    let mut attempts = 0;
    const MAX_ATTEMPTS: u8 = 10;

    while attempts < MAX_ATTEMPTS {
        let data_result = client.get_account_data(&TREASURY_ADDRESS).await;
        match data_result {
            Ok(data) => match Treasury::try_from_bytes(&data) {
                Ok(treasury) => return *treasury,
                Err(_) => {
                    eprintln!("Failed to parse treasury account data.");
                    attempts += 1;
                }
            },
            Err(e) => {
                eprintln!("Attempt {} failed: {:?}", attempts + 1, e);
                attempts += 1;
            }
        }

        if attempts < MAX_ATTEMPTS {
            sleep(Duration::from_secs(5)).await; // Wait before retrying
        } else {
            panic!("Failed to retrieve and parse treasury data after {} attempts.", MAX_ATTEMPTS);
        }
    }

    panic!("This point should not be reachable; indicates a logical error in the retry loop.");
}

pub async fn get_proof(cluster: String, authority: Pubkey) -> Proof {
    let client = RpcClient::new_with_commitment(cluster, CommitmentConfig::confirmed());
    let proof_address = proof_pubkey(authority);
    let data = client
        .get_account_data(&proof_address)
        .await
        .expect("Failed to get miner account");
    *Proof::try_from_bytes(&data).expect("Failed to parse miner account")
}

pub async fn get_clock_account(cluster: String) -> Clock {
    let client = RpcClient::new_with_commitment(cluster, CommitmentConfig::confirmed());
    let data = client
        .get_account_data(&sysvar::clock::ID)
        .await
        .expect("Failed to get miner account");
    bincode::deserialize::<Clock>(&data).expect("Failed to deserialize clock")
}

#[cached]
pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ore::ID).0
}

#[cached]
pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}
