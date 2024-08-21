use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<ModelListData>,
}

/// See: https://docs.mistral.ai/api/#tag/models
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelListData {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub owned_by: String,
    pub root: Option<String>,
    pub archived: bool,
    pub name: String,
    pub description: String,
    pub capabilities: ModelListDataCapabilies,
    pub max_context_length: u32,
    pub aliases: Vec<String>,
    /// ISO 8601 date (`YYYY-MM-DDTHH:MM:SSZ`).
    pub deprecation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelListDataCapabilies {
    pub completion_chat: bool,
    pub completion_fim: bool,
    pub function_calling: bool,
    pub fine_tuning: bool,
}
