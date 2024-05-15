use std::{clone, ptr::copy};

use serde_json::json;
use reqwest::Client;
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


pub async fn ask_open_ai(text: &str) -> String {
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

#[tauri::command]
pub async fn get_correction(text: String) -> OpenAiCompletion{
    let body = json!({
      "model":"gpt-3.5-turbo",
      "messages": [
        {
          "role": "system",
          "content": "You're an editor checking for typos, misspelling errors or grammar errors in my articles. \n
          Find those errors in the text you receive. \n
          Return the result in a json formatted string. \n
          The json keys for each error should be: \n
          location: the section, title or subtitle of the article where the error si located, \n
          errorType: the type of the error: should be one of [Misspelling, Grammar, Capitalization, Clarity], \n
          error: the wrong test,\n
          correction: the correct text"
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
  .json(&body).send().await.unwrap();

  let open_ai_response: OpenAiCompletion = response.json().await.unwrap();

  return open_ai_response;
}

