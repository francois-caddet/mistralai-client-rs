use serde::{Deserialize, Serialize};

pub const API_URL_BASE: &str = "https://api.mistral.ai/v1";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "open-mistral-7b")]
    OpenMistral7b,
    #[serde(rename = "open-mixtral-8x7b")]
    OpenMixtral8x7b,
    #[serde(rename = "open-mixtral-8x22b")]
    OpenMixtral8x22b,
    #[serde(rename = "open-mistral-nemo", alias = "open-mistral-nemo-2407")]
    OpenMistralNemo,
    #[serde(rename = "mistral-tiny")]
    MistralTiny,
    #[serde(rename = "mistral-small-latest", alias = "mistral-small-2402")]
    MistralSmallLatest,
    #[serde(rename = "mistral-medium-latest", alias = "mistral-medium-2312")]
    MistralMediumLatest,
    #[serde(rename = "mistral-large-latest", alias = "mistral-large-2407")]
    MistralLargeLatest,
    #[serde(rename = "mistral-large-2402")]
    MistralLarge,
    #[serde(rename = "codestral-latest", alias = "codestral-2405")]
    CodestralLatest,
    #[serde(rename = "open-codestral-mamba")]
    CodestralMamba,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum EmbedModel {
    #[serde(rename = "mistral-embed")]
    MistralEmbed,
}
