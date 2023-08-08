use dotenvy::dotenv;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let req = ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("hello openai"),
        }],
    };
    let result = client.chat_completion(req).await?;

    println!("{:?}", result.choices[0].message.content);
    Ok(())
}
