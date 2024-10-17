use serde_json::{json, Value};

pub fn get_translation(
    text: &str, 
    source_language: &str, 
    target_language: &str
) -> Result<String, anyhow::Error> {
    // TODO move to parameters
    let client = reqwest::blocking::Client::new();
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("Environment variable ANTHROPIC_API_KEY is not set");
  
    let prompt = format!(
        "Translate the following text from source language to target language. 
        Only provide the translation, no explanations:

        Source language: {}
        Target language: {}
        Text: {} 
        ",
        source_language, target_language, text
    );
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-3-haiku-20240307",
            "max_tokens": 1024,
            "messages": [{
                "role": "user",
                "content": prompt
            }]
        }))
        .send()?;

    let response_text: Value = serde_json::from_str(&response.text()?)?;
    let translation = match response_text["content"][0]["text"].as_str() {
        Some(translation) => translation,
        None => panic!("Could not get translation from response."),
    };

    Ok(translation.to_string())
}