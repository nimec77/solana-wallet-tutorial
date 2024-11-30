use clap::Parser;
use solana_client::rpc_client::RpcClient;

mod cli_parser;
mod cluster_info;
mod lamports;
mod generate_keys;
mod balance;
mod airdrop;
mod transfer;

const SERVER_URL: &str = "https://api.devnet.solana.com";

fn main() {
    let cli = cli_parser::Cli::parse();
    let client = RpcClient::new(SERVER_URL.to_string());

    cli_parser::parse_cli(&cli, &client);
}
