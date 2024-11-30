use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub(super) fn get_balance(address: &str, client: &RpcClient) {
    let pub_key = Pubkey::from_str(address).unwrap();
    let balance_value = client.get_balance(&pub_key).unwrap();

    println!("Balance for {address}: {}", lamports_to_sol(balance_value));
}
