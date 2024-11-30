use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::io::Write;
use std::str::FromStr;
use std::{io, thread};

pub(super) fn transfer_sol(client: &RpcClient, from_wallet: &str, to_address: &str, sol_amount: f64) {
    let keypair = read_keypair_file(from_wallet).unwrap();
    let to_pub_key = Pubkey::from_str(to_address).unwrap();
    let lamports = sol_to_lamports(sol_amount);
    let transfer_instruction =
        system_instruction::transfer(&keypair.pubkey(), &to_pub_key, lamports);
    let last_block_hash = client.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        last_block_hash,
    );

    let wait_millis = std::time::Duration::from_millis(100);
    print!("Wait to confirm");
    io::stdout().flush().unwrap();

    match client.send_transaction(&transaction) {
        Ok(signature) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&signature) {
                if confirmed {
                    println!("\nTransfer to {to_address}: {confirmed}");
                    break;
                }
            }
            print!(".");
            io::stdout().flush().unwrap();
            thread::sleep(wait_millis);
        },
        Err(e) => {
            println!("Error: transferring sol: {e}");
        }
    }
}
