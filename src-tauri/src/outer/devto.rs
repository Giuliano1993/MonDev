
use serde_json::json;
use reqwest::Client;
use crate::outer::utils::get_secret_backend;

#[tauri::command]
pub async fn create_article(content: &str) -> Result<String, ()> {
    let api_key = get_secret_backend("devtoApi");
    let body = json!({
        "article": {
        "title": "titolo bozza",
        "body_markdown": "content",
        "published": false,
        "series": "MonDEV",
        "main_image": "",
        "canonical_url": "",
        "description": "",
        "tags": "",
        "organization_id": 0
        }
    });

    println!("{}",&body);
    let response_json = Client::new()
    .post("https://dev.to/api/articles")
    .header("api-key",api_key)
    .header("Accept","application/json")
    .json(&body).send().await;
    let mut response;
    match response_json{
        Ok(respose) => {
            println!("{:?}",respose);
            response = respose.text().await.unwrap();
                
        },
        Err(_)=>panic!("{}", 2)
    }

    //.unwrap().text().await.unwrap();
    
    println!("{}",response);

    //format!("{}","response")
        Ok(response)

}        

    