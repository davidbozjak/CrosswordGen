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
        "model": "gpt-6-luna",
        "messages": [
            {
                "role": "system",
                "content": "You are a masterfull crossword designer designed to output JSON with human readable text in Swedish. Always respond in Swedish. Use natural Swedish and avoid direct translations from English."
            },
            {
                "role": "user",
                "content": format!("Generate 5 or more short clues in the swedish language to which the response is a swedish word/term. The answer is: '{answer}'. Like on other crossword puzzles, the clue can lead to a swedish word or term or phrase. If you choose to interpret the answer as several words squeezed together without a space indicate that in the clue. You can also choose to interpret it as a word in another language than swedish, however in those cases always indicate which language in the clue. Never include the provided answer in the clue you generate.")
            },
            {
                "role": "user",
                "content": "Generate a confidence score for each of the clues you generate on the range of 0-100 signaling how good and understandable the clue is to a normal swedish speaker."
            },
            {
                "role": "user",
                "content": "Respond in JSON using schema {clue, confidence_score}. Return only valid JSON. Do not use markdown. Do not wrap the response in ``` blocks."
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

#[cfg(test)]
mod tests {
    use crate::entry::Entry;
    
    use super::*;

    #[cfg(feature = "prompt_test-manual")]
    #[test]
    fn test_ai_propt() {
        let config = Config::load();
        let answer = "lia".to_string();
        let entry = get_ai_clue(answer, &config).unwrap();

        println!("answer: {}, clue: {}, confidence: {}", entry.get_key(), entry.get_clue(), entry.get_confidence());

        assert!(!entry.get_clue().is_empty());
    }
}