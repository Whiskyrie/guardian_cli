use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn list(
    role: Option<&str>,
    search: Option<&str>,
    format: &output::OutputFormat,
) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in".to_string()))?;
    let client = GuardianClient::with_token(config, &token)?;

    let mut variables = serde_json::json!({});
    if let Some(role) = role {
        variables["role"] = serde_json::Value::String(role.to_uppercase());
    }
    if let Some(search) = search {
        variables["search"] = serde_json::Value::String(search.to_string());
    }

    let response: models::UsersResponse = client
        .graphql_request(queries::USERS, Some(variables), Some(&token))
        .await?;

    let users: Vec<models::User> = response
        .users
        .edges
        .into_iter()
        .map(|e| e.node)
        .collect();

    output::print_users_list(&users, format);
    Ok(())
}

pub async fn create(
    email: &str,
    password: &str,
    first: &str,
    last: &str,
    format: &output::OutputFormat,
) -> Result<()> {
    let config = Config::load()?;
    let client = GuardianClient::new(config)?;

    let variables = serde_json::json!({
        "email": email,
        "password": password,
        "firstName": first,
        "lastName": last,
    });

    let response: models::RegisterResponse = client
        .graphql_request(queries::REGISTER_USER, Some(variables), None)
        .await?;

    let payload = response.register_user;

    if payload.success {
        output::print_message(
            payload.message.as_deref().unwrap_or("User created"),
            true,
            format,
        );
        if let Some(user) = &payload.user {
            output::print_user(user, format);
        }
    } else {
        output::print_message(
            payload.message.as_deref().unwrap_or("Failed to create user"),
            false,
            format,
        );
        output::print_errors(&payload.errors, format);
    }

    Ok(())
}

pub async fn delete(id: &str, format: &output::OutputFormat) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in".to_string()))?;
    let client = GuardianClient::with_token(config, &token)?;

    let variables = serde_json::json!({ "id": id });

    let response: models::DeleteUserResponse = client
        .graphql_request(queries::DELETE_USER, Some(variables), Some(&token))
        .await?;

    let payload = response.delete_user;

    if payload.success {
        output::print_message(
            payload.message.as_deref().unwrap_or("User deleted"),
            true,
            format,
        );
    } else {
        output::print_message(
            payload.message.as_deref().unwrap_or("Failed to delete user"),
            false,
            format,
        );
        output::print_errors(&payload.errors, format);
    }

    Ok(())
}
