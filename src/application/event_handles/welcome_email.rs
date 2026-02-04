use crate::domain::events::{error::KafkaResult, handler::EventHandler, user::UserCreated};
use async_trait::async_trait;
use tracing::info;

#[derive(Default)]
pub struct WelcomeEmailHandler;

#[async_trait]
impl EventHandler for WelcomeEmailHandler {
    async fn handle(&self, payload: &str) -> KafkaResult<()> {
        let event: UserCreated = serde_json::from_str(payload)?;

        info!(
            "📧 [Welcome Email] Sending welcome email to {} ({})",
            event.user_id, event.email
        );

        // Simulate sending email (e.g., call EmailService)
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        info!(
            "✅ [Welcome Email] Email sent successfully to {}",
            event.email
        );

        Ok(())
    }
}
