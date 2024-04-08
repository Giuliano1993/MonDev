// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod outer;
use outer::brevo::create_campaing;
use outer::openai::ask_open_ai;
use outer::openai::OpenAiCompletion;
use outer::utils::{save_secret, get_secret};
use tauri::CustomMenuItem;
use tauri::Menu;
use serde_json::{json,Value};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
async fn translate(text: String) -> String {
    
    //&text[..]-> it's a pattern that allows to convert from String to str
    // it takes a full slice of the string, char by char, creating the str
    let openai_response = ask_open_ai(&text[..]).await;
    let translation_response: OpenAiCompletion = serde_json::from_str(&openai_response).unwrap();
    format!("{}",translation_response.choices[0].message.content)

}

#[tauri::command]
async fn create_brevo_campaign()->String{
  let response = create_campaing().await;
  let creation_response: Value = serde_json::from_str(&response).unwrap();
  format!("{}", creation_response)
}

fn main() {

  let config = CustomMenuItem::new("config".to_string(), "Config");

  let menu = Menu::new().add_item(config);
    tauri::Builder::default()
      .menu(menu)
      .on_menu_event(|event|{
        match event.menu_item_id(){
            "config" => {
                println!("ciaone");
            },
            _ => {}
        }
        
      }) 
        .invoke_handler(tauri::generate_handler![translate,get_secret,save_secret,create_brevo_campaign])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
