use dotenv::dotenv;
use near_api::prelude::*;
use near_crypto::SecretKey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();
    let private_key_string = std::env::var("PRIVATE_KEY").unwrap();
    let account_id_string = std::env::var("ACCOUNT_ID").unwrap();

    let account_id: AccountId = account_id_string.parse().unwrap();

    let private_key = SecretKey::from_str(&private_key_string).unwrap();
    let signer = Signer::new(Signer::secret_key(private_key)).unwrap();

    // Create a connection to the NEAR testnet
    let mut network = NetworkConfig::testnet();

    // Change the RPC URL optional
    network.rpc_url = "https://rpc.testnet.near.org".parse().unwrap();

    // Send NEAR tokens to another account
    let send_tokens_result = Tokens::of(account_id.clone()) // example-account.testnet
        .send_to("receiver-account.testnet".parse().unwrap())
        .near(NearToken::from_near(1))
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();
    println!("{:?}", send_tokens_result);
}
