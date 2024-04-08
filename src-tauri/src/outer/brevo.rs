use serde_json::json;
use reqwest::{Client};
use serde::{Serialize, Deserialize};

use crate::outer::utils::get_secret_backend;


pub async fn create_campaing() -> String {

    let api_key = get_secret_backend("brevoApi");
    let body = json!({
        "name":"test-campaign",
        "subject":"test",
        "sender":{
         "name":"mondev",
         "email":"mondev@ghostylab.com"   
        },
        "previewText":"testo di anteprima",
        "recipients":{
            "listIds":[8]
        },
        "htmlContent":"<h1>Hello World</h1>",

    });
    let response = Client::new()
    .post("https://api.brevo.com/v3/emailCampaigns")
    .header("api-key", api_key)
    .json(&body).send().await.unwrap().text().await.unwrap();

    format!("{}",response)

}