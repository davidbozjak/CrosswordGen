use crate::config::Config;
use crate::entry::Entry;

use reqwest::blocking::Client;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AIGeneratedClue {
    clue: String,
    confidence_score: u32,
}

pub fn get_clue(answer:String, config: &Config) -> Result<Entry, Box<dyn std::error::Error>> {
    let mut fetch_from_api: bool = false;

    #[cfg(not(debug_assertions))]
    {
        fetch_from_api = true;
    }

    if fetch_from_api {
        get_ai_clue(answer, &config)
    }
    else {
        Ok(Entry::new(answer, "Placeholder clue".to_string(), Some(80)))
    }
}

fn get_ai_clue(answer:String, config: &Config) -> Result<Entry, Box<dyn std::error::Error>> {
    let endpoint = &config.ai_gen_url;
    let api_key = &config.ai_gen_key;

    let url = format!(
        "{}/chat/completions",
        endpoint
    );

    let body = json!({
        "model": "gpt-4.1-mini",
        "messages": [
            {
                "role": "system",
                "content": "You are a masterfull crossword designer designed to output JSON with human readable text in Swedish. Always respond in Swedish. Use natural Swedish and avoid direct translations from English."
            },
            {
                "role": "user",
                "content": format!("Generate 5 or more short, clever and qurky clues to which the resposne is '{answer}'. Generate a confidence score for each on the range of 0-100 signifying how likely a normal swedish speaker is likely to understand this clue. Never include the provided answer in the clue you generate.")
            },
            {
                "role": "user",
                "content": "Respond in JSON using schme {clue, confidence_score}. Return only valid JSON. Do not use markdown. Do not wrap the response in ``` blocks."
            }
        ]
    });

    let json_response: serde_json::Value = Client::new()
        .post(url)
        .header("api-key", api_key)
        .json(&body)
        .send()?
        .json()?;

    let json_content = json_response["choices"][0]["message"]["content"].as_str().unwrap();

    let mut results: Vec<AIGeneratedClue> =
        serde_json::from_str(&json_content)?;

    results.sort_by_key(|w| w.confidence_score);

    let best = &results.last().unwrap();

    Ok(Entry::new(answer, best.clue.clone(), Some(best.confidence_score)))
}