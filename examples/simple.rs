use near_api::prelude::*;

#[tokio::main]
async fn main() {
    // Connecting to NEAR
    let mut network = NetworkConfig::testnet();

    // Change the RPC URL optional
    network.rpc_url = "https://rpc.testnet.near.org".parse().unwrap();
    
    // RPC failover
    // TODO?

    // Create account object
    let my_account_id: AccountId = "a-whole-new-account.testnet".parse().unwrap();
    let my_account = Account(my_account_id.clone());

    // View near balance
    let _near_balance = Tokens::of(my_account_id.clone())
        .near_balance()
        .fetch_from(&network)
        .await
        .unwrap();

    // Get details
    // Method doesn't exist 

    // Get state
    let _account_info = my_account.view().fetch_from(&network).await.unwrap();
}
