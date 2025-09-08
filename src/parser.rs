//parser 
use serde_json::Value;
use std::fs;
use crate::models::ChatMessage;
use crate::errors::CustomError;


pub fn parse_message(item: &Value) -> Result<ChatMessage, CustomError> {
    let channel_name = item["channel_name"].as_str().ok_or(CustomError::ParseError("Missing channel_name".to_string()))?.to_string();
    let language = item["language"].as_str().ok_or(CustomError::ParseError("Missing language".to_string()))?.to_string();
    let viewer_count = item["viewer_count"].as_u64().ok_or(CustomError::ParseError("Missing or invalid viewer_count".to_string()))? as i32;

    Ok(ChatMessage {
        channel_name,
        language,
        viewer_count,
    })
}
pub fn parse_json_file(path: &str) -> Result<Vec<ChatMessage>, CustomError> {
   
    // read file content
    let content =  fs::read_to_string(path).map_err(|e| CustomError::IOError(e.to_string()))?;

    // parse json content
    let json : Value = serde_json::from_str(&content).map_err(|e| CustomError::ParseError(e.to_string()))?;

    // extract manually 
    let root_array = json["root"].as_array().ok_or(CustomError::ParseError("No root array found".to_string()))?;

    let mut messages = Vec::new();

    for item in root_array {
        let message = parse_message(item)?;
        messages.push(message);
    }

    Ok(messages)

 }