use dotenv::dotenv;
use near_api::prelude::{AccountId, NearToken, NetworkConfig, Signer, Tokens};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let account_id_string = std::env::var("ACCOUNT_ID").unwrap();

    let account_id: AccountId = account_id_string.parse().unwrap();

    // Create a signer via the credentials directory
    let credentials_path = std::path::PathBuf::from("credentials-file.json"); // Path relative to the root directory of the project
    let signer = Signer::new(Signer::access_keyfile(credentials_path).unwrap()).unwrap();

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
