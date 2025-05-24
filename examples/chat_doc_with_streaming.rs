use futures::stream::StreamExt;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    client::Client,
    constants::Model,
    doc::Doc,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::MistralSmall;
    let messages = vec![ChatMessage::new_user_message(vec![
        "What is the purpose of this doc?".into(),
        Doc::from_url("https://arxiv.org/pdf/1805.04770").into(),
    ])];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };

    let stream_result = client
        .chat_stream(model, messages, Some(options))
        .await
        .unwrap();
    stream_result
        .for_each(|chunk_result| async {
            match chunk_result {
                Ok(chunk) => {
                    print!("{}", chunk.choices[0].delta.content.as_ref().unwrap());
                    io::stdout().flush().unwrap();
                }
                Err(error) => {
                    eprintln!("Error processing chunk: {:?}", error)
                }
            }
        })
        .await;
    print!("\n") // To persist the last chunk output.
                 // => "Once upon a time, [...]"
}
