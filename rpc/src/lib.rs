//! Kakarot RPC module for Ethereum.
//! It is an adapter layer to interact with Kakarot ZK-EVM.
use std::net::SocketAddr;
mod eth_rpc;
use eth_rpc::KakarotEthRpc;
use eyre::Result;
use jsonrpsee::server::ServerBuilder;
use reth_rpc_api::EthApiServer;

pub async fn run_server() -> Result<SocketAddr> {
    let server = ServerBuilder::default().build("127.0.0.1:0").await?;

    let addr = server.local_addr()?;
    let handle = server.start(KakarotEthRpc.into_rpc())?;

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}
