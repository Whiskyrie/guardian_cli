use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn assign(
    user_id: &str,
    roles: &[String],
    format: &output::OutputFormat,
) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in".to_string()))?;
    let client = GuardianClient::with_token(config, &token)?;

    let roles_upper: Vec<String> = roles.iter().map(|r| r.to_uppercase()).collect();
    let variables = serde_json::json!({
        "userId": user_id,
        "roleNames": roles_upper,
    });

    let response: models::UpdateUserRoleResponse = client
        .graphql_request(queries::UPDATE_USER_ROLE, Some(variables), Some(&token))
        .await?;

    let payload = response.update_user_role;

    if payload.success {
        output::print_message(&payload.message, true, format);
        if let Some(user) = &payload.user {
            output::print_user(user, format);
        }
    } else {
        output::print_message(&payload.message, false, format);
        output::print_errors(&payload.errors, format);
    }

    Ok(())
}
