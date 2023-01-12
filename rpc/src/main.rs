use eyre::Result;
use futures::future::pending;
use kakarot_rpc::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    let server_addr = run_server().await?;
    println!("{server_addr}");
    pending::<()>().await;
    Ok(())
}
