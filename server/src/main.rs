use dotenv::dotenv;
use server::{create_payment_router, set_up_tracing_subscriber, ApiSettings};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use tracing::{error, event, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    set_up_tracing_subscriber();

    let app = create_payment_router().await?;
    let api = ApiSettings::build().map_err(|e| {
        error!("{}", e);
        e
    })?;
    let socket_addr_v4 = SocketAddrV4::new(Ipv4Addr::from_str(&api.host)?, api.port.parse()?);
    let listener = tokio::net::TcpListener::bind(socket_addr_v4).await?;
    event!(Level::INFO, "Application Started");
    event!(Level::INFO, "Running on {0}", socket_addr_v4);
    axum::serve(listener, app).await?;

    Ok(())
}
