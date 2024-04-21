use core::panic;
use std::fs;

use serde_json::json;
use reqwest::Client;

use crate::outer::utils::get_secret_backend;


#[tauri::command]
pub async fn create_campaing(name: String, subject: String, preview_text: String, list: u8, mut content: String) -> String {

    let api_key = get_secret_backend("brevoApi");
    let template = fs::read_to_string("./template.html");
    match template {
        Ok(template_string)=>{
          content = template_string.replace("[[theTitle]]",&preview_text).replace("[[theContent]]", &content); 
        },
        Err(_)=>panic!("Oh No")
    }
    let body = json!({
        "name":name,
        "subject":subject,
        "sender":{
         "name":"mondev",
         "email":"mondev@ghostylab.com"   
        },
        "previewText":preview_text,
        "recipients":{
            "listIds":[list]
        },
        "htmlContent":content,

    });
    
    let response = Client::new()
    .post("https://api.brevo.com/v3/emailCampaigns")
    .header("api-key", api_key)
    .json(&body).send().await.unwrap().text().await.unwrap();

    format!("{}",response)

}