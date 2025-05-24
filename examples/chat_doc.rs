use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    client::Client,
    constants::Model,
    doc::Doc,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::Pixtral;
    let messages = vec![ChatMessage::new_user_message(vec![
        "What is the purpose of this doc?".into(),
        Doc::from_url("https://arxiv.org/pdf/1805.04770").into(),
    ])];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.chat(model, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
