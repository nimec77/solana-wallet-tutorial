use chrono::{DateTime, Utc};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, sysvar, account::from_account};
use solana_sdk::clock::Clock;

pub(super) fn get_cluster_info(client: &RpcClient) {
    let version = client.get_version().unwrap();
    let result = client
        .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
        .unwrap();

    let (slot, timestamp) = match result.value {
        Some(clock_account) => {
            let clock: Clock = from_account(&clock_account).unwrap();

            (result.context.slot, clock.unix_timestamp)
        },
        None => {
            panic!("Unexpected None");
        }
    };

    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();
    let local_datetime = datetime.with_timezone(&chrono::Local);

    println!("Cluster version: {}", version.solana_core);
    println!("Block: {slot}, Time: {}", local_datetime.format("%Y-%m-%d %H:%M:%S"));
}
