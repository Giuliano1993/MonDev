use std::{fs, io::{self, ErrorKind}, str::FromStr};
use serde_json::{json,Value};


#[tauri::command]
pub fn save_secret(key: &str, value: &str) -> String{
    let file_content = fs::read("./keys.txt");
    let mut json = keys_to_json(file_content);

    json[key] = json!(value);

    let new_content = serde_json::to_string(&json);
    match new_content {
        Ok(str_value) => fs::write("./keys.txt", str_value),
        Err(_)=>panic!("oh no")
    };
    format!("Saved succesfully")
}


#[tauri::command]
pub fn get_secret(key: &str) ->String{
    let keys = fs::read("./keys.txt");
    let json = keys_to_json(keys);

    let key_val = json.get(key);
    let mut value = "";
    match key_val {
        Some(val)=>value = val.as_str().unwrap(),
        None => () 
    }
    format!("{}",value)
}

pub fn get_secret_backend(key: &str) -> String {

    let keys = fs::read("./keys.txt");
    let json = keys_to_json(keys);
    return String::from_str(json[key].as_str().unwrap()).unwrap() ;
}


pub fn keys_to_json(keys: Result<Vec<u8>, io::Error>) -> Value {
    let json : Value;
    match keys {
        Ok(keys_content)=>{
           let string_keys = String::from_utf8(keys_content).unwrap();
           let json_result: Result<Value, serde_json::Error>  = serde_json::from_str(&string_keys);
           match json_result {
               Ok(json_val)=> json = json_val,
               Err(_)=> json = Value::from_str("{}").unwrap()
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