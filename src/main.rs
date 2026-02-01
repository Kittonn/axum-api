use anyhow::Result;
use axum_api::infra::{app::create_app, setup::init_app_state};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let app_state = init_app_state().await?;
    let app = create_app(app_state.clone());

    let addr = format!("0.0.0.0:{}", app_state.config.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
