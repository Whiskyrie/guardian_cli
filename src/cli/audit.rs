use crate::api::{models, queries, GuardianClient};
use crate::config::Config;
use crate::error::{GuardianError, Result};
use crate::output;

pub async fn list(
    action: Option<&str>,
    user_id: Option<&str>,
    recent_hours: Option<i32>,
    format: &output::OutputFormat,
) -> Result<()> {
    let config = Config::load()?;
    let token = Config::load_token()?.ok_or_else(|| GuardianError::Auth("Not logged in".to_string()))?;
    let client = GuardianClient::with_token(config, &token)?;

    let mut variables = serde_json::json!({});
    if let Some(action) = action {
        variables["action"] = serde_json::Value::String(action.to_string());
    }
    if let Some(user_id) = user_id {
        variables["userId"] = serde_json::Value::String(user_id.to_string());
    }
    if let Some(hours) = recent_hours {
        variables["recentHours"] = serde_json::Value::Number(hours.into());
    }

    let response: models::AuditLogsResponse = client
        .graphql_request(queries::AUDIT_LOGS, Some(variables), Some(&token))
        .await?;

    let logs: Vec<models::AuditLog> = response
        .audit_logs
        .edges
        .into_iter()
        .map(|e| e.node)
        .collect();

    output::print_audit_logs(&logs, format);
    Ok(())
}
