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

    let beneficiary_account_id: AccountId = account_id_string.parse().unwrap();

    let private_key = SecretKey::from_str(&private_key_string).unwrap();
    let creator_signer = Signer::new(Signer::secret_key(private_key)).unwrap();

    let network = NetworkConfig::testnet();

    // First create a new account to be deleted
    let delete_account_id = generate_testnet_account_id();
    let (delete_account_private_key, create_account_tx) = Account::create_account()
        .fund_myself(
            delete_account_id.clone(),
            beneficiary_account_id.clone(),
            NearToken::from_millinear(100),
        )
        .new_keypair()
        .generate_secret_key()
        .unwrap();

    create_account_tx
        .with_signer(creator_signer.clone()) // Signer is the account that is creating the new account
        .send_to(&network)
        .await
        .unwrap();

    // Create an account object for the new account
    // and a new signer
    let account_to_delete = Account(delete_account_id.clone());
    let signer = Signer::new(Signer::secret_key(delete_account_private_key)).unwrap();

    // Delete the account with account Id of the account object
    let delete_account_result = account_to_delete
        .delete_account_with_beneficiary(beneficiary_account_id.clone()) // example-beneficiary.testnet
        .with_signer(signer.clone()) // Signer is the account that is being deleted
        .send_to(&network)
        .await
        .unwrap();
    println!("{:?}", delete_account_result);
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
