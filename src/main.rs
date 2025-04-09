mod chatter;
use chatter::*;
use std::{
	env,
	net::{Ipv4Addr, TcpListener},
};

use openai_api_rs::v1::api::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api_key: String = env::var("OPENAI_API_KEY").unwrap().to_string();
	let website_ip: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);

	let listener = TcpListener::bind("0.0.0.0:8080")?;
	let mut connections: u64 = 0;

	for mut stream in listener.incoming().filter_map(|c| c.ok()) {
		if stream.peer_addr().unwrap().is_ipv4()
			&& stream
				.peer_addr()
				.unwrap()
				.ip()
				.to_canonical()
				.eq(&website_ip)
		//TODO! ipv6
		{
			todo!();
		}
		connections += 1;
		println!(
			"connection num: {} from: {}",
			connections,
			stream.peer_addr().ok().unwrap().to_string()
		);

		let val = api_key.clone();

		_ = tokio::spawn(async move {
			let client = OpenAIClient::builder().with_api_key(val).build().unwrap();
			run(client, &mut stream).await;
			println!("finished connection num: {}", connections);
		});
	}

	Ok(())
}
