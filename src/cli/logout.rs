use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn run(format: &output::OutputFormat) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in".to_string()))?;

    let client = GuardianClient::with_token(config, &token)?;
    let response: models::LogoutResponse = client
        .graphql_request(queries::LOGOUT_USER, None, Some(&token))
        .await?;

    let payload = response.logout_user;

    if payload.success {
        Config::clear_token()?;
        output::print_message(
            payload.message.as_deref().unwrap_or("Logged out"),
            true,
            format,
        );
    } else {
        output::print_message(
            payload.message.as_deref().unwrap_or("Logout failed"),
            false,
            format,
        );
        output::print_errors(&payload.errors, format);
    }

    Ok(())
}
