use validator::ValidationError;

use crate::features::users::model::UserRole;

pub fn validate_user_role(role: &UserRole) -> Result<(), ValidationError> {
    match role {
        UserRole::Admin | UserRole::Guest | UserRole::User => Ok(()),
        _ => Err(ValidationError::new("invalid_role")),
    }
}
