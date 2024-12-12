use near_api::prelude::{AccountId, NearToken, NetworkConfig, Signer, Tokens};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let seed_phrase_string = std::env::var("SEED_PHRASE").unwrap();
    let account_id_string = std::env::var("ACCOUNT_ID").unwrap();

    let account_id: AccountId = account_id_string.parse().unwrap();

    // Create a signer from the private key string
    let seed_phrase = Signer::seed_phrase(seed_phrase_string, None).unwrap(); // No password
    let signer = Signer::new(seed_phrase).unwrap(); // Create the signer

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
