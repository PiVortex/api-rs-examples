use near_api::{prelude::*, signer::secret_key::SecretKeySigner};
use near_crypto::SecretKey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let network = NetworkConfig::testnet();

    // Create account object
    let my_account_id: AccountId = "a-whole-new-account.testnet".parse().unwrap();

    // Create signer object from private key
    // Need to use near_crypto::SecretKey
    let private_key = SecretKey::from_str("ed25519:3bUTUXCPHPbAD5JDukzsWT5BaJ9iZA3FF9wLgYgRvzC7CDYMgmEExtxyGjnGATvmM3oggqUErvRkN9sjzNTD8yd7").unwrap();
    let signer = Signer::new(SecretKeySigner::new(private_key)).unwrap();

    // Create account in a standard way
    let new_account_id_1: AccountId = "some-account-sdifj.testnet".parse().unwrap();

    let res = Account::create_account()
        .fund_myself(new_account_id_1.clone(), my_account_id.clone(), NearToken::from_near(1)) // Also have sponsor by faucet and implicit
        .new_keypair() 
        .save_generated_seed_to_file("./new_account_seed".into()) 
        .unwrap()
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();


    // Create an account with a load of custom stuff 
    let new_account_id_2: AccountId = "some-account-s4fitwe.testnet".parse().unwrap();
    
    let (seed_phrase, res) = Account::create_account()
        .fund_myself(new_account_id_2.clone(), my_account_id.clone(), NearToken::from_near(1))
        .new_keypair()
        .word_count(24)
        .generate_seed_phrase()
        .unwrap();

    println!("{:?}", seed_phrase);

    let res = res.with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    println!("{:?}", res);

    let new_account_id_3: AccountId = "suba-.a-whole-new-account.testnet".parse().unwrap();

    // Create sub account 
    let res = Account::create_account()
        .fund_myself(new_account_id_3.clone(), my_account_id.clone(), NearToken::from_near(1)) // Also have sponsor by faucet and implicit
        .new_keypair() 
        .save_generated_seed_to_file("./new_account_seed".into()) 
        .unwrap()
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
