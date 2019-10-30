    extern crate serenity;
    extern crate reqwest;
    extern crate serde_json;

    use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    };
    use std::str::FromStr;
    use std::collections::HashMap;

    const _COMMANDS: [&str; 7] = ["Command List:\n", "!test", "!add", "!gif", "!nextsession", "!setnextsession", "!commands"];

    pub fn test(context: &Context, message: &Message) {
        if let Err(why) = message.channel_id.say(&context.http, "I hear ya!") {
                println!("Error! Message failed to send: {:?}", why);
        }
    }

    pub fn add(context: &Context, message: &Message, message_content: &Vec<&str>) {
        let mut outbound_message: String = String::new();
        if message_content.len() < 3 {
            outbound_message = String::from("Not enough arguments!! Should be like this: !add num1 num2...numX");
        } else {
            let mut result: f32 = 0.0;
            for i in 1..message_content.len() {
                result = result + f32::from_str(message_content[i]).unwrap_or_else( |err| {
                    eprintln!("Failure in addition: {:?}", err);
                    outbound_message = String::from("Invalid Parameters");
                    0.0
                })
            }
            outbound_message = String::from("Result of addition: ");
            outbound_message.push_str(&result.to_string()[..]);
        }
        if let Err(why) = message.channel_id.say(&context.http, outbound_message) {
            eprintln!("Error! Message failed to send: {:?}", why);
        }
    }

    pub fn gif(context: &Context, message: &Message, message_content: &mut Vec<&str>, api_key: &String) {
        let giphy_url = "https://api.giphy.com/v1/gifs/random?api_key=";
        let tag = { 
            message_content.remove(0);
            &message_content.join("%20")[..]
        };
        let rating = "g";
        let request_vector = vec![giphy_url, api_key, "&tag=", tag, "&rating=", rating];
        let request_string = request_vector.join("");
        eprintln!("Request String: {:?}", request_string);
        let gif_response_text: String = reqwest::get(&request_string[..]).unwrap_or_else( |error| {
            eprintln!("Error on request: {:?}", error);
            panic!("Bad Response! {:?}", error);
        }).text().unwrap();
        let gif_response_json: serde_json::value::Value = serde_json::from_str(&gif_response_text).unwrap();
        let gif_json_urls: &serde_json::value::Value = &gif_response_json["data"];
        //eprintln!("Response: {:?}", gif_response_json);
        if let Err(why) = message.channel_id.say(&context.http, gif_json_urls["bitly_gif_url"].as_str().unwrap()) {
            eprintln!("Couldn't send gif: {:?}", why);
        }
    }

    pub fn next_session(context: &Context, message: &Message, next_session: &String) {
        let output_vec = vec!["Next session scheduled for: ", &next_session];
        let output = output_vec.join("");
        if let Err(why) = message.channel_id.say(&context.http, &output) {
            eprintln!("Couldn't send session details: {:?}", why);
        }
    }

    pub fn set_next_session(context: &Context, message: &Message, config: &crate::configuration::Config) {
        let output_vec = vec!["Next session updated to: ", &config.next_session];
        let output = output_vec.join("");
        if let Err(why) = message.channel_id.say(&context.http, &output) {
            eprintln!("Couldn't send confirmation of session set: {:?}", why);
        }
    }

    pub fn commands(context: &Context, message: &Message) {
        let iter = _COMMANDS.into_iter();
        let mut output_vector: Vec<&str> = Vec::new();
        for cmd in iter {
            output_vector.push(cmd);
        }
        if let Err(why) = message.channel_id.say(&context.http, output_vector.join(" ")) {
            eprintln!("Couldn't send commands: {:?}", why);
        }
    }