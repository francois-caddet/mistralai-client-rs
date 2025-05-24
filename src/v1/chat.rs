use std::fmt;

use serde::{Deserialize, Serialize};

use crate::v1::{common, constants, tool};

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatMessageRole,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<tool::ToolCall>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Content {
    Simple(String),
    Vec(Vec<ContentElm>),
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Simple(text) => write!(f, "{}", text),
            _ => todo!(),
        }
    }
}

impl From<&str> for Content {
    fn from(text: &str) -> Self {
        Content::Simple(text.to_string())
    }
}

impl From<Vec<ContentElm>> for Content {
    fn from(content: Vec<ContentElm>) -> Self {
        Content::Vec(content)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentElm {
    ImageUrl {
        image_url: super::img::Img,
    },
    DocumentUrl {
        #[serde(flatten)]
        document_url: super::doc::Doc,
    },
    Text {
        text: String,
    },
}

impl From<&str> for ContentElm {
    fn from(text: &str) -> Self {
        ContentElm::Text {
            text: text.to_string(),
        }
    }
}

impl From<super::img::Img> for ContentElm {
    fn from(img: super::img::Img) -> Self {
        ContentElm::ImageUrl { image_url: img }
    }
}

impl From<super::doc::Doc> for ContentElm {
    fn from(doc: super::doc::Doc) -> Self {
        ContentElm::DocumentUrl { document_url: doc }
    }
}

impl ChatMessage {
    pub fn new_assistant_message(
        content: impl Into<Content>,
        tool_calls: Option<Vec<tool::ToolCall>>,
    ) -> Self {
        Self {
            role: ChatMessageRole::Assistant,
            content: content.into(),
            tool_call_id: None,
            tool_calls,
        }
    }

    pub fn new_user_message(content: impl Into<Content>) -> Self {
        Self {
            role: ChatMessageRole::User,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn new_tool_message(content: &str, id: Option<String>) -> Self {
        Self {
            role: ChatMessageRole::Tool,
            content: content.into(),
            tool_calls: None,
            tool_call_id: id,
        }
    }

    pub fn new_system_message(content: &str) -> Self {
        Self {
            role: ChatMessageRole::System,
            content: content.into(),
            tool_call_id: None,
            tool_calls: None,
        }
    }
}

use tool::ToolResult;
impl From<ToolResult> for ChatMessage {
    fn from(ToolResult { id, name, result }: ToolResult) -> Self {
        let content =
            result.unwrap_or_else(|e| format!("Error trying to execute tool `{name}`: {e}"));
        Self::new_tool_message(&content, id)
    }
}

/// See the [Mistral AI API documentation](https://docs.mistral.ai/capabilities/completion/#chat-messages) for more information.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChatMessageRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "tool")]
    Tool,
}

/// The format that the model must output.
///
/// See the [API documentation](https://docs.mistral.ai/api/#operation/createChatCompletion) for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
}
impl ResponseFormat {
    pub fn json_object() -> Self {
        Self {
            type_: "json_object".to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// Request

/// The parameters for the chat request.
///
/// See the [API documentation](https://docs.mistral.ai/api/#operation/createChatCompletion) for more information.
#[derive(Clone, Debug)]
pub struct ChatParams {
    /// The maximum number of tokens to generate in the completion.
    ///
    /// Defaults to `None`.
    pub max_tokens: Option<u32>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    ///
    /// Defaults to `None`.
    pub random_seed: Option<u32>,
    /// The format that the model must output.
    ///
    /// Defaults to `None`.
    pub response_format: Option<ResponseFormat>,
    /// Whether to inject a safety prompt before all conversations.
    ///
    /// Defaults to `false`.
    pub safe_prompt: bool,
    /// What sampling temperature to use, between `Some(0.0)` and `Some(1.0)`.
    ///
    /// Defaults to `0.7`.
    pub temperature: f32,
    /// Specifies if/how functions are called.
    ///
    /// Defaults to `None`.
    pub tool_choice: Option<tool::ToolChoice>,
    /// A list of available tools for the model.
    ///
    /// Defaults to `None`.
    pub tools: Option<Vec<tool::Tool>>,
    /// Nucleus sampling, where the model considers the results of the tokens with `top_p` probability mass.
    ///
    /// Defaults to `1.0`.
    pub top_p: f32,
}
impl Default for ChatParams {
    fn default() -> Self {
        Self {
            max_tokens: None,
            random_seed: None,
            safe_prompt: false,
            response_format: None,
            temperature: 0.7,
            tool_choice: None,
            tools: None,
            top_p: 1.0,
        }
    }
}
impl ChatParams {
    pub fn json_default() -> Self {
        Self {
            max_tokens: None,
            random_seed: None,
            safe_prompt: false,
            response_format: None,
            temperature: 0.7,
            tool_choice: None,
            tools: None,
            top_p: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: constants::Model,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    pub safe_prompt: bool,
    pub stream: bool,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<tool::ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<tool::Tool>>,
    pub top_p: f32,
}
impl ChatRequest {
    pub fn new(
        model: constants::Model,
        messages: Vec<ChatMessage>,
        stream: bool,
        options: Option<ChatParams>,
    ) -> Self {
        let ChatParams {
            max_tokens,
            random_seed,
            safe_prompt,
            temperature,
            tool_choice,
            tools,
            top_p,
            response_format,
        } = options.unwrap_or_default();

        Self {
            messages,
            model,

            max_tokens,
            random_seed,
            safe_prompt,
            stream,
            temperature,
            tool_choice,
            tools,
            top_p,
            response_format,
        }
    }
}

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: constants::Model,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: common::ResponseUsage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatResponseChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: ChatResponseChoiceFinishReason,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChatResponseChoiceFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
