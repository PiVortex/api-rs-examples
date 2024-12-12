use dotenv::dotenv;
use near_api::prelude::{AccountId, NearToken, NetworkConfig, Signer, Tokens};
use near_crypto::SecretKey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let private_key_string = std::env::var("PRIVATE_KEY").unwrap();
    let account_id_string = std::env::var("ACCOUNT_ID").unwrap();

    let account_id: AccountId = account_id_string.parse().unwrap();

    // Create a signer from the private key string
    let private_key = SecretKey::from_str(&private_key_string).unwrap(); // ed25519::5Fg2...
    let signer = Signer::new(Signer::secret_key(private_key)).unwrap(); // Create the signer

    let network = NetworkConfig::testnet();

    // Test the signer by transferring NEAR
    Tokens::of(account_id.clone()) // example-account.testnet
        .send_to("receiver-account.testnet".parse().unwrap())
        .near(NearToken::from_near(1))
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap()
        .assert_success();
}
