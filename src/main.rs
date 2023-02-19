use reqwest::header::{HeaderMap, USER_AGENT};
use serde::Deserialize;
use std::error::Error;
use dotenv::dotenv;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
struct Article {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct NewsApiResponse {
    articles: Vec<Article>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("NEWS_API_KEY")?;
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "reqwest".parse().unwrap());
    let resp = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let response_data: NewsApiResponse = serde_json::from_str(&resp)?;

    println!("Articles:");
    for article in response_data.articles {
        println!("Title: {} \nLink: {} \n--------------------", article.title, article.url);
    }
    Ok(())
}
