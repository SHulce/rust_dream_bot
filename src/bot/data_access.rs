extern crate serde;
extern crate serde_json;

use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub fn search(file_name: &String, query: &String) -> String {
    eprintln!("Query: {}", query);
    eprintln!("Filename: {}", file_name);
    let file_content: String = fs::read_to_string(file_name).unwrap_or_else( |file_name| {
        eprintln!("Failed to read file {} for search", file_name);
        String::from("FAILED")
    });

    let output: String = if file_content != String::from("FAILED") {
        let file_map: serde_json::Value = serde_json::from_str(&file_content).unwrap_or_else(|err| {
            eprintln!("Failed to generate JSON: {:?}", err);
            serde_json::json!("{\"ERROR\": \"Failed to parse file\"}")
        });
        serde_json::to_string_pretty(&file_map.get(query)).unwrap_or_else( |err| {
            eprintln!("String Prettification failed: {:?}", err);
            String::from("Failed to Generate String properly. Contact administrator.")
        })

    } else {
        String::from("Failed to read file. Contact moderator for assistance if you believe that your query was correct.")
    };
    return output;
}