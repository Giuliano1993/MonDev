use serde_json::json;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::outer::utils::get_secret_backend;

#[derive(Serialize, Deserialize)]
pub struct OpenAiMessage {
    role: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct OpenAiChoice {
    pub message: OpenAiMessage,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAiCompletion {
    pub choices: Vec<OpenAiChoice>
}


pub async fn askOpenAi(text: &str) -> String {
    let body = json!({
        "model":"gpt-3.5-turbo",
        "messages": [
          {
            "role": "system",
            "content": "Translate the text you receive from italian to english. Return only the translated text. Format it as unparsed markdown."
          },
          {
            "role":"user",
            "content":text
          }
        ]
   });
   let  open_ai_key : String= get_secret_backend("openAiSecret");
    let response = Client::new()
    .post("https://api.openai.com/v1/chat/completions")
    .bearer_auth(open_ai_key)
    .json(&body).send().await.unwrap().text().await.unwrap();

    format!("{}",response)

}

