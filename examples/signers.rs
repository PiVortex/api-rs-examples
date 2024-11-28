use near_api::prelude::*;
use near_api::signer::keystore::KeystoreSigner;
use near_crypto::{SecretKey, PublicKey};
use near_primitives::account::AccessKeyPermission;
use std::str::FromStr;
use std::path::Path;
use std::env;

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
    let home_dir = env::var("HOME").unwrap();
    let credentials_dir = Path::new(&home_dir).join(".near-credentials");
    let file_path = credentials_dir.join(format!("testnet/{}.json", my_account_id));
    let file_signer = Signer::new(Signer::access_keyfile(file_path).unwrap()).unwrap();

    // test file signer
    Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap()
        .1
        .with_signer(file_signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // keystore specifying public key
    let public_key = PublicKey::from_str("ed25519:G5k73gFDHjFQTaHp26zShbcMt2Quth2sJJfqra1eetuT").unwrap();
    let keystore_signer = Signer::new(Signer::keystore(public_key)).unwrap();

    // test keystore signer
    Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap()
        .1
        .with_signer(keystore_signer.clone())
        .send_to(&network)
        .await
        .unwrap();

    // Keystore not specifying public key
    let search_keystore_signer = KeystoreSigner::search_for_keys(my_account_id.clone(), &network).await.unwrap();
    let keystore_signer_search = Signer::new(search_keystore_signer).unwrap();

    // test keystore signer search
    Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap()
        .1
        .with_signer(keystore_signer_search.clone())
        .send_to(&network)
        .await
        .unwrap();

    // using seed phrase
    let seed_phrase = "shoe sell gate jelly half tissue parrot robust census lens staff ship".to_string();
    let seed_phrase_signer = Signer::new(Signer::seed_phrase(seed_phrase, None).unwrap()).unwrap();

    // test seed phrase signer
    Account(my_account_id.clone())
        .add_key(AccessKeyPermission::FullAccess)
        .new_keypair()
        .generate_secret_key()
        .unwrap()
        .1
        .with_signer(seed_phrase_signer.clone())
        .send_to(&network)
        .await
        .unwrap();
}

// Leave out ledger, not common + don't want to attach my own!
// There is also seed_phrase_with_hd_path, also this and seed_phrase have the options for password