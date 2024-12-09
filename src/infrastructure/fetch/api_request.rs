use serde_json::Value;

use super::FetchError;

pub(super) trait ApiRequest {
    async fn api_call(&self, url: &str) -> Result<Value, FetchError> {
        let resp = reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|e| FetchError::SendRequest(e.to_string()))?;
        let status = resp.status();
        if status.is_client_error() {
            Err(FetchError::Client(status.to_string()))?
        } else if status.is_server_error() {
            Err(FetchError::Server(status.to_string()))?
        }
        let resp_value: Value =
            resp.json().await.map_err(|e| FetchError::Deserialize(e.to_string()))?;
        Ok(resp_value)
    }

    /// end of string is `/`
    const BASE_API_URL: &str = "https://www.googleapis.com/youtube/v3/";
}
