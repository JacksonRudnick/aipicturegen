use core::panic;
use std::env;
use reqwest::Client;
use tokio::runtime::Runtime;
use serde_json::json;

fn main() {
	let prompt = String::from("Pacman");

	let rt = Runtime::new().unwrap();

	rt.block_on(async {
		let data = send_request(prompt).await;

		match data {
			Ok(response) => {
				print!("{}", response);
			}
			Err(_err) => {
				panic!("error with reponse")
			}
		}
	});
}

async fn send_request(prompt: String) -> Result<String, reqwest::Error> {
	let width = 1000;
	let height = 1000;

	let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set.");
	let gpt3_endpoint = "https://api.openai.com/v1/engines/gpt-3.5-turbo-instruct/completions/";

	let request_body = json!({"prompt": format!("Draw {} with {} by {} pixels in jpeg format ", prompt, width, height)});

	let client = Client::new();
	let response = client
		.post(gpt3_endpoint)
		.header("Content-Type", "application/json")
		.header("Authorization", format!("Bearer {}", api_key))
		.body(serde_json::to_string(&request_body).unwrap())
		.send()
		.await?;

	let response_text = response.text().await?;

	Ok(response_text)
}