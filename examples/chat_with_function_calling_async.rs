use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    client::Client,
    constants::Model,
    tool::{Function, Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};
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

    let model = Model::Ministral3b;
    let mut messages = vec![ChatMessage::new_user_message(
        "What's the temperature in Paris?",
    )];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Auto),
        tools: Some(tools),
        ..Default::default()
    };

    let res = client
        .chat_async(model.clone(), messages.clone(), Some(options.clone()))
        .await
        .unwrap();
    messages.push(res.choices[0].message.clone());
    let temperature = client.get_last_function_call_result().unwrap();
    messages.push(temperature.into());
    let res = client
        .chat_async(model, messages, Some(options))
        .await
        .unwrap();
    println!("{}", res.choices[0].message.content);
    // => "The temperature in Paris is: 20°C."
}
