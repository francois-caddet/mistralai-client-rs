use futures::stream::StreamExt;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    chat_stream::ChatStreamChunk,
    client::Client,
    constants::Model,
    error::ApiError,
    img::Img,
    tool::{Function, Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};
use std::io::{self, Write};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetCityTemperatureArguments {
    city: String,
}

#[derive(Debug)]
struct GetCityTemperatureFunction;
#[async_trait::async_trait]
impl Function for GetCityTemperatureFunction {
    type Args = GetCityTemperatureArguments;
    type Result = String;
    async fn call(&self, arguments: Self::Args) -> Self::Result {
        match arguments.city.as_str() {
            "Paris" => "20°C",
            _ => "Unknown city",
        }
        .to_string()
    }
}

async fn print_and_aggregate(
    agg: ChatMessage,
    chunk_result: Result<ChatStreamChunk, ApiError>,
) -> ChatMessage {
    match chunk_result {
        Ok(chunk) => {
            let reason = chunk.choices[0].finish_reason.clone();
            if Some("stop".to_string()) == reason {
                println!("\n");
            }
            let mut delta = chunk.choices[0].delta.clone();
            let delta_content = delta
                .content
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default();
            print!("{}", delta_content);
            io::stdout().flush().unwrap();
            // => "Once upon a time, [...]"
            let tool_calls = if let Some(mut current_tools) = agg.tool_calls {
                delta.tool_calls.as_mut().map(|new_tools| {
                    current_tools.append(new_tools);
                    current_tools
                })
            } else {
                delta.tool_calls
            };
            ChatMessage::new_assistant_message(
                (agg.content.to_string() + &delta_content).as_str(),
                tool_calls,
            )
        }
        Err(error) => {
            eprintln!("Error processing chunk: {:?}", error);
            agg
        }
    }
}
#[tokio::main]
async fn main() {
    let tools = vec![Tool::new(
        "get_city_temperature".to_string(),
        "Get the current temperature in a city.".to_string(),
        vec![ToolFunctionParameter::new(
            "city".to_string(),
            "The name of the city.".to_string(),
            ToolFunctionParameterType::String,
        )],
    )];

    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let mut client = Client::new(None, None, None, None).unwrap();
    client.register_function("get_city_temperature", GetCityTemperatureFunction);

    let model = Model::PixtralLarge;
    let mut messages = vec![ChatMessage::new_user_message(
        vec![
        "Checking the current temperature in Paris, describe this photo and say if it could have been taken today.".into(),
            Img::from_url("https://upload.wikimedia.org/wikipedia/commons/b/b8/Luigi_Loir_-_Paris_sous_la_neige.jpg").into(),
        ]
    )];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Any),
        tools: Some(tools),
        ..Default::default()
    };

    let stream_result = client
        .chat_stream(model.clone(), messages.clone(), Some(options.clone()))
        .await
        .unwrap();
    let res = stream_result
        .fold(
            ChatMessage::new_assistant_message("", None),
            print_and_aggregate,
        )
        .await;
    messages.push(res);
    messages.push(client.get_last_function_call_result().unwrap().into());
    let stream_result = client
        .chat_stream(
            model,
            messages,
            Some(ChatParams {
                tool_choice: Some(ToolChoice::None),
                ..options
            }),
        )
        .await
        .unwrap();
    stream_result
        .fold(
            ChatMessage::new_assistant_message("", None),
            print_and_aggregate,
        )
        .await;
}
