use crate::generate_keys::generate_keypair;
use crate::{cluster_info, lamports};
use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;

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
        None => {
            println!("No command provided");
        }
    }
}