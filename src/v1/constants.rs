use serde::{Deserialize, Serialize};

pub const API_URL_BASE: &str = "https://api.mistral.ai/v1";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "pixtral-large-2411")]
    PixtralLarge,
    #[serde(rename = "pixtral-12b-2409")]
    Pixtral,
    #[serde(rename = "ministral-3b-2410")]
    Ministral3b,
    #[serde(rename = "ministral-8b-2410")]
    Ministral8b,
    #[serde(rename = "mistral-saba-2502")]
    MistralSaba,
    #[serde(rename = "open-mistral-nemo", alias = "open-mistral-nemo-2407")]
    MistralNemo,
    #[serde(rename = "mistral-small-2501")]
    MistralSmall,
    #[serde(rename = "mistral-large-2411")]
    MistralLarge,
    #[serde(rename = "codestral-2501")]
    Codestral,
    #[serde(rename = "open-codestral-mamba")]
    CodestralMamba,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum EmbedModel {
    #[serde(rename = "mistral-embed")]
    MistralEmbed,
}
