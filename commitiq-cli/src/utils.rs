use reqwest;
use reqwest::Error;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct GPT3Request {
    prompt: String,
    max_tokens: usize,
    model: String,
}

#[derive(Deserialize, Debug)]
struct GPT3Response {
    choices: Vec<GPT3ResponseChoice>,
}

#[derive(Deserialize, Debug)]
struct GPT3ResponseChoice {
    text: String,
}

async fn is_valid_api_key(api_key: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.openai.com/v1/engines")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    Ok(response.status() == StatusCode::OK)
}

pub async fn get_gpt3_response(api_key: &str, prompt: &str) -> Result<String, Error> {
    if !is_valid_api_key(api_key).await? {
        println!("Invalid API key!");
        println!("Please run ciq config set <OPENAI_API_KEY> to set your API key.");
        std::process::exit(1);
    }

    let client = reqwest::Client::new();
    let req_body = GPT3Request {
        prompt: prompt.to_string(),
        max_tokens: 100,
        model: "text-davinci-003".to_string(),
    };

    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&req_body)
        .send()
        .await?;

    if res.status() != StatusCode::OK {
        println!("API call failed with status code: {}", res.status());
        return Ok("".to_string());
    }

    let data: GPT3Response = res.json().await?;
    let commit_message = data.choices[0].text.clone();

    Ok(commit_message.trim().to_string())
}
