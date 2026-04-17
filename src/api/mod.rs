pub mod models;
pub mod queries;

use crate::error::{GuardianError, Result};
use crate::config::Config;
use models::{GraphqlBody, GraphqlResponse};
use reqwest::Client;

pub struct GuardianClient {
    client: Client,
    config: Config,
}

impl GuardianClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        Ok(Self { client, config })
    }

    pub fn with_token(config: Config, token: &str) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                let bearer = format!("Bearer {}", token);
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&bearer)
                        .unwrap_or_else(|_| reqwest::header::HeaderValue::from_static("")),
                );
                headers
            })
            .build()?;
        Ok(Self { client, config })
    }

    pub async fn graphql_request<T: serde::de::DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
        token: Option<&str>,
    ) -> Result<T> {
        let body = GraphqlBody {
            query: query.to_string(),
            variables,
        };

        let mut request = self.client.post(self.config.graphql_url()).json(&body);

        if let Some(tok) = token {
            request = request.header("Authorization", format!("Bearer {}", tok));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(GuardianError::Api(format!("HTTP {}: {}", status, text)));
        }

        let graphql_resp: GraphqlResponse<T> = response.json().await?;

        if let Some(errors) = graphql_resp.errors {
            let messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(GuardianError::Graphql(messages.join("; ")));
        }

        graphql_resp
            .data
            .ok_or_else(|| GuardianError::Graphql("No data in response".to_string()))
    }
}
