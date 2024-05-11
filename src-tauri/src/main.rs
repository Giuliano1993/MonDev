// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod outer;
use outer::brevo::create_campaing;
use outer::openai::{ask_open_ai,get_correction};
use outer::openai::OpenAiCompletion;
use outer::devto::create_article;
use outer::utils::{save_secret, get_secret};
use tauri::CustomMenuItem;
use serde_json;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
async fn translate(text: String) -> String {
    
    //&text[..]-> it's a pattern that allows to convert from String to str
    // it takes a full slice of the string, char by char, creating the str
    let openai_response = ask_open_ai(&text[..]).await;
    let translation_response: OpenAiCompletion = serde_json::from_str(&openai_response).unwrap();
    format!("{}",translation_response.choices[0].message.content)

}



fn main() {

  let config = CustomMenuItem::new("config".to_string(), "Config");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          translate,
          get_secret,
          save_secret,
          create_campaing,
          create_article,
          get_correction
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
