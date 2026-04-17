use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn run(format: &output::OutputFormat) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in. Run `guardian login` first.".to_string()))?;

    let client = GuardianClient::with_token(config, &token)?;
    let variables = serde_json::json!({ "token": token });

    let response: models::RefreshTokenResponse = client
        .graphql_request(queries::REFRESH_TOKEN, Some(variables), Some(&token))
        .await?;

    let payload = response.refresh_token;

    if payload.success {
        if let Some(new_token) = &payload.token {
            Config::save_token(new_token)?;
        }
        output::print_message(
            payload.message.as_deref().unwrap_or("Token refreshed"),
            true,
            format,
        );
    } else {
        output::print_message(
            payload.message.as_deref().unwrap_or("Token refresh failed"),
            false,
            format,
        );
        output::print_errors(&payload.errors, format);
    }

    Ok(())
}
