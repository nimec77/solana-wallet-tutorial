use solana_client::rpc_client::RpcClient;

pub(super) fn get_supply(client: &RpcClient) {
    let supply_response = client.supply().unwrap();
    let supply = supply_response.value;

    println!(
        "Total supply: {} SOL\nCirculating supply: {} SOL\nNon-circulating supply: {} SOL",
        supply.total,
        supply.circulating,
        supply.non_circulating
    )
}