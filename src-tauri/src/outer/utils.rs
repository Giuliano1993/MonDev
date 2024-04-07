use std::{fs, io::{self, BufRead, ErrorKind}, str::FromStr};
use anyhow::Error;
use serde_json::{json, to_string_pretty, Value};


pub fn saveSecret(key: &str) -> String{
    fs::write("./keys.txt", "hello");
    format!("secret saved")
}


#[tauri::command]
pub fn get_secret(key: &str) ->String{
    let keys = fs::read("./keys.txt");
    let json = keys_to_json(keys);

    format!("{}",json[key])
    //format!("ciao")
}

pub fn keys_to_json(keys: Result<Vec<u8>, io::Error>) -> Value {
    let json : Value;
    match keys {
        Ok(keys_content)=>{
           let string_keys = String::from_utf8(keys_content).unwrap();
           let json_result: Result<Value, serde_json::Error>  = serde_json::from_str(&string_keys);
           match json_result {
               Ok(json_val)=> json = json_val,
               Err(e)=> json = Value::from_str("{}").unwrap()
           }
        },
        Err(error)=>{
            match error.kind() {
                ErrorKind::NotFound => {
                  fs::write("./keys.txt", "");  
                    json = serde_json::from_str("{}").unwrap();
                },
                _ => {
                    json = serde_json::from_str("{}").unwrap();
                }
            }
        }
    }
    return json;
}