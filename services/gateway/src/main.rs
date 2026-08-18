use flock_signal_gateway::{app, AppConfig};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();
    if config.api_key.is_none() {
        eprintln!(
            "warning: FLOCK_SIGNAL_API_KEY is not configured; /v1 routes will deny all requests"
        );
    }
    let listener = TcpListener::bind(&config.bind).await?;
    println!("flock-signal-gateway listening on {}", config.bind);
    axum::serve(listener, app(config.api_key)).await?;
    Ok(())
}
