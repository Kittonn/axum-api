use anyhow::Result;
use axum_api::infra::app::create_app;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let app = create_app();

    let listener = TcpListener::bind("127.0.0.1:4001").await?;

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
