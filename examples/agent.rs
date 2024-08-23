use mistralai_client::v1::{
    agent::{AgentMessage, AgentMessageRole, AgentParams},
    client::Client,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let agid = std::env::var("MISTRAL_API_AGENT")
        .expect("Please export MISTRAL_API_AGENT with the agent id you want to use");
    let messages = vec![AgentMessage {
        role: AgentMessageRole::User,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
        tool_calls: None,
    }];
    let options = AgentParams {
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.agent(agid, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
