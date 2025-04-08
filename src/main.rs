use std::{env, error::Error};

use openai_api_rs::v1::{
	api::OpenAIClient,
	chat_completion::{self, ChatCompletionRequest},
	common::GPT4_O,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
	let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;
	let req = ChatCompletionRequest::new(
		GPT4_O.to_string(),
		vec![chat_completion::ChatCompletionMessage {
			role: chat_completion::MessageRole::user,
			content: chat_completion::Content::Text(String::from("What is bitcoin?")),
			name: None,
			tool_calls: None,
			tool_call_id: None,
		}],
	);

	let result = client.chat_completion(req).await.unwrap();
	println!("Content: {:?}", result.choices[0].message.content);

	for (key, value) in client.headers.unwrap().iter() {
		println!("{}: {:?}", key, value);
	}
	Ok(())
}
