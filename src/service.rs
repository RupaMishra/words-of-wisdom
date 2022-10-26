use reqwest;
use serde_json;

pub async fn get_quotes(){
	let data = reqwest::get("https://api.quotable.io/random").await.unwrap().text().await.unwrap();
	let quote: serde_json::Value = serde_json::from_str(&data).unwrap();
	let content = quote["content"].as_str().unwrap();
	let author = quote["author"].as_str().unwrap();

	println!("{} - {}", content, author);
}