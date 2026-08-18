use flock_signal_gateway::{app, AppConfig};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();
    let listener = TcpListener::bind(&config.bind).await?;
    println!("flock-signal-gateway listening on {}", config.bind);
    axum::serve(listener, app()).await?;
    Ok(())
}
