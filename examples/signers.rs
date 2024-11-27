use near_api::prelude::*;
use near_crypto::SecretKey;
use near_primitives::account::AccessKeyPermission;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let network = NetworkConfig::testnet();

    // Specify account Id
    let my_account_id: AccountId = "a-whole-new-account.testnet".parse().unwrap();

    let private_key = SecretKey::from_str("ed25519:3bUTUXCPHPbAD5JDukzsWT5BaJ9iZA3FF9wLgYgRvzC7CDYMgmEExtxyGjnGATvmM3oggqUErvRkN9sjzNTD8yd7").unwrap();
    let priv_key_signer = Signer::new(Signer::secret_key(private_key)).unwrap();

    // test private key signer
    Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap()
        .1
        .with_signer(priv_key_signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // access key file
    // let file_signer = Signer::new(Signer::access_keyfile(./))
}

// Leave out ledger, not common + don't want to attach my own!
