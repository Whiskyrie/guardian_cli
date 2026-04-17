use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn run(format: &output::OutputFormat) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in. Run `guardian login` first.".to_string()))?;

    let client = GuardianClient::with_token(config, &token)?;
    let response: models::CurrentUserResponse = client
        .graphql_request(queries::CURRENT_USER, None, Some(&token))
        .await?;

    match response.current_user {
        Some(user) => output::print_user(&user, format),
        None => output::print_message("No authenticated user found", false, format),
    }

    Ok(())
}
