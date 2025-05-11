use reqwest::Client;
use serde_json::{json, Value};
use std::env;

pub async fn paraphrase_text(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("MISTRAL_API_KEY").expect("MISTRAL_API_KEY must be set");
    let client = Client::new();

    let response = client
        .post("https://api.mistral.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "mistral-tiny",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant that paraphrases text while maintaining its original meaning."
                },
                {
                    "role": "user",
                    "content": format!("Please paraphrase the following text: {}", text)
                }
            ]
        }))
        .send()
        .await?;

    let response_json: Value = response.json().await?;
    let paraphrased_text = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Failed to paraphrase text")
        .to_string();

    Ok(paraphrased_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_paraphrase_text() {
        dotenv().ok();
        let test_text = "Hello world, this is a test.";
        let result = paraphrase_text(test_text).await;
        assert!(result.is_ok(), "Paraphrase request should succeed");

        let paraphrased = result.unwrap();
        assert!(
            !paraphrased.is_empty(),
            "Paraphrased text should not be empty"
        );
        assert_ne!(
            paraphrased, test_text,
            "Paraphrased text should be different from input"
        );
    }
}
