use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(uuid).is_err() {
        let mut error = ValidationError::new("uuid");
        error.message = Some("Invalid format".into());
        return Err(error);
    }

    Ok(())
}
