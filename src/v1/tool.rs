use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use thiserror::Error;

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ToolCall {
    pub id: Option<String>,
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tool {
    pub r#type: ToolType,
    pub function: ToolFunction,
}
impl Tool {
    pub fn new(
        function_name: String,
        function_description: String,
        function_parameters: Vec<ToolFunctionParameter>,
    ) -> Self {
        let properties: HashMap<String, ToolFunctionParameterProperty> = function_parameters
            .into_iter()
            .map(|param| {
                (
                    param.name,
                    ToolFunctionParameterProperty {
                        r#type: param.r#type,
                        description: param.description,
                    },
                )
            })
            .collect();
        let property_names = properties.keys().cloned().collect();

        let parameters = ToolFunctionParameters {
            r#type: ToolFunctionParametersType::Object,
            properties,
            required: property_names,
        };

        Self {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: function_name,
                description: function_description,
                parameters,
            },
        }
    }
}

// -----------------------------------------------------------------------------
// Request

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunction {
    name: String,
    description: String,
    parameters: ToolFunctionParameters,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameter {
    name: String,
    description: String,
    r#type: ToolFunctionParameterType,
}
impl ToolFunctionParameter {
    pub fn new(name: String, description: String, r#type: ToolFunctionParameterType) -> Self {
        Self {
            name,
            r#type,
            description,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameters {
    r#type: ToolFunctionParametersType,
    properties: HashMap<String, ToolFunctionParameterProperty>,
    required: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameterProperty {
    r#type: ToolFunctionParameterType,
    description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolFunctionParametersType {
    #[serde(rename = "object")]
    Object,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolFunctionParameterType {
    #[serde(rename = "string")]
    String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

/// An enum representing how functions should be called.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolChoice {
    /// The model is forced to call a function.
    #[serde(rename = "any")]
    Any,
    /// The model can choose to either generate a message or call a function.
    #[serde(rename = "auto")]
    Auto,
    /// The model won't call a function and will generate a message instead.
    #[serde(rename = "none")]
    None,
}

/// The struct returned by last_function_call_results which can be converted
/// into a toll message response.
#[derive(Debug)]
pub struct ToolResult {
    pub id: Option<String>,
    pub name: String,
    pub result: Result<String, CallError>,
}

#[derive(Debug, Error)]
pub enum CallError {
    #[error("Tool does not exist")]
    NotFound,
}

// -----------------------------------------------------------------------------
// Custom

#[async_trait]
pub trait Function: Send + Sync + Debug {
    type Args: for<'de> Deserialize<'de>;
    type Result: Serialize;
    async fn call(&self, args: Self::Args) -> Self::Result;
}

#[async_trait]
pub trait DynFunction: Debug {
    async fn execute(&self, args: String) -> String;
}

#[async_trait]
impl<A, R, F: Function<Args = A, Result = R>> DynFunction for F
where
    A: for<'de> Deserialize<'de>,
    R: Serialize,
{
    async fn execute(&self, args: String) -> String {
        let args = serde_json::from_str(&args).unwrap();
        let res = self.call(args).await;
        serde_json::to_string(&res).unwrap()
    }
}
