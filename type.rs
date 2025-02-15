
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingWebhookParams {
    pub webhook_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PingWebhookResponse {
    pub error: Option<String>,

    pub status: String,
}