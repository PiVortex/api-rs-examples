use near_api::prelude::{Account, AccountId, NearToken, NetworkConfig, Signer};
use near_crypto::SecretKey;
use rand::{thread_rng, Rng};
use dotenv::dotenv;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let private_key_string = std::env::var("PRIVATE_KEY").unwrap();
    let account_id_string = std::env::var("ACCOUNT_ID").unwrap();

    let account_id: AccountId = account_id_string.parse().unwrap();

    let private_key = SecretKey::from_str(&private_key_string).unwrap();
    let signer = Signer::new(Signer::secret_key(private_key)).unwrap();

    let network = NetworkConfig::testnet();

    // Create a .testnet account with seed phrase
    let new_account_id = generate_testnet_account_id();
    let (seed_phrase, create_account_tx) = Account::create_account()
        .fund_myself(
            new_account_id.clone(), // example-account.testnet
            account_id.clone(),
            NearToken::from_millinear(100), // Initial balance for new account in yoctoNEAR
        )
        .new_keypair()
        .generate_seed_phrase()
        .unwrap();

    println!("Seed phrase: {:?}", seed_phrase);

    let create_account_result = create_account_tx
        .with_signer(signer.clone()) // Signer is the account that is creating the new account
        .send_to(&network)
        .await
        .unwrap();
    println!("{:?}", create_account_result);
}

// Random account ID generator
fn generate_testnet_account_id() -> AccountId {
    let random_string: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) // Allow only lowercase and digits
        .take(8)
        .map(char::from)
        .collect();
    format!("{}.testnet", random_string).parse().unwrap()
}
