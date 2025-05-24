use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    client::Client,
    constants::Model,
    img::Img,
};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::Pixtral;
    let messages = vec![ChatMessage::new_user_message(vec![
        "Describe this image: ".into(),
        Img::from_file("./img.png").into(),
    ])];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client
        .chat_async(model, messages, Some(options))
        .await
        .unwrap();
    println!(
        "{:?}: {}",
        result.choices[0].message.role, result.choices[0].message.content
    );
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
