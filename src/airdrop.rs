use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use std::io::Write;
use std::str::FromStr;
use std::{io, time};

pub(super) fn airdrop_sol(address: &str, sol: f64, client: &RpcClient) {
    let lamports = sol_to_lamports(sol);
    let pub_key = Pubkey::from_str(address).unwrap();
    let signature = client.request_airdrop(&pub_key, lamports).unwrap();

    let wait_millis = time::Duration::from_millis(100);
    println!("Waiting for confirm");
    io::stdout().flush().unwrap();
    loop {
        if let Ok(confirmed) = client.confirm_transaction(&signature) {
            if confirmed {
                println!("\nAirdrop to {address}: {confirmed}");
                break;
            }
        }
        println!(".");
        io::stdout().flush().unwrap();
        std::thread::sleep(wait_millis);
    }
}
