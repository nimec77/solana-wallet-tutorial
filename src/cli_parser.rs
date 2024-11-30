use crate::generate_keys::generate_keypair;
use crate::{cluster_info, lamports};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use crate::airdrop::airdrop_sol;
use crate::balance::get_balance;
use crate::transfer::transfer_sol;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub(super) enum Commands {
    ClusterInfo,
    Supply,
    KeyGen {
        #[arg[short, long, help = "Output file path for keypair file."]]
        output: String,
        #[arg(
        short,
        long,
        default_value_t = 12,
        help = "How many words to generate for the mnemonic. Valid values are 12, 15, 18, 21, and 24."
        )]
        mnemonic_word_count: u32,
        #[arg(short, long, help = "Passphrase to use for extra security.")]
        passphrase: Option<String>,
    },
    Balance {
        #[arg(long, group = "input")]
        address: Option<String>,
        #[arg(long, group = "input")]
        wallet_file: Option<String>,
    },
    Airdrop {
        #[arg(short, long)]
        address: String,
        #[arg(short, long, help = "Wallet file path.")]
        amount: f64,
    },
    Transfer {
        #[arg(short, long)]
        from_wallet: String,
        #[arg(short, long)]
        to_address: String,
        #[arg(short, long)]
        amount: f64,
    }
}

pub(super) fn parse_cli(cli: &Cli, client: &RpcClient)  {
    match &cli.command {
        Some(Commands::ClusterInfo) => {
            println!("ClusterInfo command");
            cluster_info::get_cluster_info(&client);
        },
        Some(Commands::Supply) => {
            println!("Supply command");
            lamports::get_supply(&client);
        },
        Some(Commands::KeyGen { output, mnemonic_word_count, passphrase }) => {
            println!("Generate keys, output: {output}");
            generate_keypair(output, *mnemonic_word_count as usize, passphrase.as_deref());
        },
        Some(Commands::Balance { address, wallet_file }) => {
            if let Some(address) = address {
                println!("Get balance for address: {address}");
                get_balance(&address, &client);
            } else if let Some(wallet_path) = wallet_file {
                println!("Get balance for Wallet file: {wallet_path}");
                let keypair = read_keypair_file(&wallet_path).unwrap();
                get_balance(&keypair.pubkey().to_string(), &client);
            }
        },
        Some(Commands::Airdrop { address, amount }) => {
            println!("Airdrop {amount} SOL to {address}");
            airdrop_sol(address, *amount, &client);
        },
        Some(Commands::Transfer { from_wallet, to_address, amount }) => {
            println!("Transfer {amount} SOL from {from_wallet} to {to_address}");
            transfer_sol(&client, from_wallet, to_address, *amount);
        },
        None => {
            println!("No command provided");
        }
    }
}