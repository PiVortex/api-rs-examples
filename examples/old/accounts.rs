use near_api::prelude::*;
use near_crypto::SecretKey;
use near_primitives::account::{AccessKeyPermission, FunctionCallPermission};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let network = NetworkConfig::testnet();

    // Specify account Id
    let my_account_id: AccountId = "a-whole-new-account.testnet".parse().unwrap();
    let my_account = Account(my_account_id.clone());

    // Create signer object from private key
    let private_key = SecretKey::from_str("ed25519:3bUTUXCPHPbAD5JDukzsWT5BaJ9iZA3FF9wLgYgRvzC7CDYMgmEExtxyGjnGATvmM3oggqUErvRkN9sjzNTD8yd7").unwrap();
    let signer = Signer::new(Signer::secret_key(private_key)).unwrap();
    let public_key = signer.get_public_key().await.unwrap();

    // Create account in a standard way, save the seed phrase to a file
    let new_account_id_1: AccountId = "some-account-sdgfj.testnet".parse().unwrap();

    let res = Account::create_account()
        .fund_myself(
            new_account_id_1.clone(),
            my_account_id.clone(),
            NearToken::from_near(1),
        )
        .new_keypair()
        .save_generated_seed_to_file("./new_account_seed".into())
        .unwrap()
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // Create an account with a load of custom stuff
    let new_account_id_2: AccountId = "some-account-s4figgwe.testnet".parse().unwrap();
    let new_account_2 = Account(new_account_id_2.clone());

    let (seed_phrase, res) = Account::create_account()
        .fund_myself(
            new_account_id_2.clone(),
            my_account_id.clone(),
            NearToken::from_near(1),
        )
        .new_keypair()
        .word_count(24)
        .generate_seed_phrase()
        .unwrap();

    let res = res
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    let signer_2 = Signer::new(Signer::seed_phrase(seed_phrase, None).unwrap()).unwrap();

    // Then delete the account account
    new_account_2
        .delete_account_with_beneficiary(my_account_id.clone())
        .with_signer(signer_2.clone())
        .send_to(&network)
        .await
        .unwrap();

    let new_account_id_3: AccountId = "subac.a-whole-new-account.testnet".parse().unwrap();

    // Create sub account
    let res = Account::create_account()
        .fund_myself(
            new_account_id_3.clone(),
            my_account_id.clone(),
            NearToken::from_near(1),
        )
        .new_keypair()
        .save_generated_seed_to_file("./new_account_seed".into())
        .unwrap()
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // Keys --------------------------------------------------------------------------------------------

    // Get a key from an account with a specific public key
    let key = my_account
        .access_key(public_key)
        .fetch_from(&network)
        .await
        .unwrap();

    // List all keys
    let keys = my_account.list_keys().fetch_from(&network).await.unwrap();

    // Add a full access key
    let (new_private_key, txn) = Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap();

    println!("New private key: {:?}", new_private_key.to_string());
    println!(
        "New public key: {:?}",
        new_private_key.public_key().to_string()
    );

    txn.with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // Add a function call key
    let new_function_call_key = AccessKeyPermission::FunctionCall(FunctionCallPermission {
        allowance: Some(250_000_000_000_000_000_000_000),
        receiver_id: "example-account.testnet".to_string(),
        method_names: vec!["example_method".to_string()],
    });

    let (new_private_key, txn) = Account(my_account_id.clone())
        .add_key(new_function_call_key)
        .new_keypair()
        .generate_secret_key()
        .unwrap();

    println!("New private key: {:?}", new_private_key.to_string());
    println!(
        "New public key: {:?}",
        new_private_key.public_key().to_string()
    );

    txn.with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // Delete a key
    my_account
        .delete_key(new_private_key.public_key())
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();
}

// A lot more can be done here
// hd path
// passphrase
// use seed phrase
// use public key
// use public key from
// generate secret key
// sponsor by faucet
// create implicit
