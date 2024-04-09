use serde_json::json;
use reqwest::{Client};
use serde::{Serialize, Deserialize};

use crate::outer::utils::get_secret_backend;


#[tauri::command]
pub async fn create_campaing(name: String, subject: String, previewText: String, list: u8, content: String) -> String {

    let api_key = get_secret_backend("brevoApi");
    let body = json!({
        "name":name,
        "subject":subject,
        "sender":{
         "name":"mondev",
         "email":"mondev@ghostylab.com"   
        },
        "previewText":previewText,
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