use tracing::{event, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .with_ansi(false)
    .init();

  event!(Level::INFO, "Application Started");

  Ok(())
}
