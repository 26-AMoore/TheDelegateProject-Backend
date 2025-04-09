use std::{io::Write, net::TcpStream};

use http::Response;
use openai_api_rs::v1::{
	api::OpenAIClient,
	chat_completion::{self, ChatCompletionRequest},
	common::GPT4_O,
};

pub async fn run(mut client: OpenAIClient, stream: &mut TcpStream) {
	let req = ChatCompletionRequest::new(
		GPT4_O.to_string(),
		vec![chat_completion::ChatCompletionMessage {
			role: chat_completion::MessageRole::user,
			content: chat_completion::Content::Text(String::from("")),
			name: None,
			tool_calls: None,
			tool_call_id: None,
		}],
	);

	let result = client.chat_completion(req).await.unwrap();
	println!("Content: {:?}", result.choices[0].message.content);

	let message = result.choices[0].message.clone();

	/*
	let response = http::Response::builder()
		.status(200)
		.body(message.content.clone().unwrap().as_bytes())
		.unwrap_or(Response::default());

	let _ = stream.write(response.status().to_string().as_bytes());

	for (key, value) in client.headers.unwrap().iter() {
		println!("{}: {:?}", key, value);
	}
	*/
}
