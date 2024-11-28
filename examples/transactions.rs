use near_api::prelude::*;
use near_crypto::SecretKey;
use std::str::FromStr;
use serde_json::json;

#[tokio::main]
async fn main() {
    let network = NetworkConfig::testnet();

    // Set up signer
    let my_account_id: AccountId = "a-whole-new-account.testnet".parse().unwrap();
    let private_key = SecretKey::from_str("ed25519:3bUTUXCPHPbAD5JDukzsWT5BaJ9iZA3FF9wLgYgRvzC7CDYMgmEExtxyGjnGATvmM3oggqUErvRkN9sjzNTD8yd7").unwrap();
    let signer = Signer::new(Signer::secret_key(private_key)).unwrap();

    // Create contract object
    let contract_id: AccountId = "guestbook.near-examples.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let another_account_id: AccountId = "pivortex.testnet".parse().unwrap();

    // Call function that requires signer
    let args = json!({
        "text": "Hello, world!"
    });

    contract
        .call_function("add_message", args)
        .unwrap()
        .transaction()
        .deposit(NearToken::from_near(1))
        .with_signer(my_account_id.clone(), signer.clone())
        .send_to(&network)
        .await
        .unwrap()
        .assert_success();

    // Call view function that doesn't require signer
    let res: Data<u32> = contract
        .call_function("total_messages", ())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await
        .unwrap();

    println!("{:?}", res.data);

    // Send near tokens
    Tokens::of(my_account_id.clone())
        .send_to(another_account_id.clone())
        .near(NearToken::from_near(1))
        .with_signer(signer.clone())
        .send_to(&network)
        .await
        .unwrap()
        .assert_success();
}
