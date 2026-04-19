use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::Result;
use crate::output;

pub async fn run(email: &str, password: Option<&str>, format: &output::OutputFormat) -> Result<()> {
    let password = match password {
        Some(p) => p.to_string(),
        None => rpassword::prompt_password("Password: ").map_err(|e| crate::error::GuardianError::Auth(e.to_string()))?,
    };

    let config = Config::load()?;
    let client = GuardianClient::new(config)?;

    let variables = serde_json::json!({
        "email": email,
        "password": password,
    });

    let response: models::LoginResponse = client
        .graphql_request(queries::LOGIN_USER, Some(variables), None)
        .await?;

    let payload = response.login_user;

    if let Some(token) = &payload.token {
        Config::save_token(token)?;
        output::print_message("Login successful", true, format);
        if let Some(user) = &payload.user {
            output::print_user(user, format);
        }
    } else {
        output::print_message("Login failed", false, format);
    }

    Ok(())
}
