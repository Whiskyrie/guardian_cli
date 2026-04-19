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

    Config::clear_token()?;
    output::print_message(
        response.logout_user.message.as_deref().unwrap_or("Logged out"),
        true,
        format,
    );

    Ok(())
}
