extern crate serenity;
extern crate reqwest;
extern crate serde_json;
extern crate rand;

use serenity::{
model::{channel::Message, gateway::Ready},
prelude::*,
};
use std::str::FromStr;
use std::collections::HashMap;
use rand::Rng;
use std::sync::RwLock;

mod configuration;

const _COMMANDS: [&str; 7] = ["Command List:\n", "!test", "!add", "!gif", "!nextsession", "!setnextsession", "!commands"];

pub struct Bot {
    config: RwLock<configuration::Config>,
}

impl Bot {

    pub fn new() -> Bot {
        Bot{config: RwLock::new(configuration::Config::new())}
    }

    pub fn test(&self, context: &Context, message: &Message) {
        if let Err(why) = message.channel_id.say(&context.http, "I hear ya!") {
            println!("Error! Message failed to send: {:?}", why);
        }
    }

    pub fn get_bot_token(&self) -> String {
        self.config.read().unwrap().bot_token.clone()
    }

    pub fn add(&self, context: &Context, message: &Message, message_content: &Vec<&str>) {
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

    pub fn gif(&self, context: &Context, message: &Message, message_content: &mut Vec<&str>) {
        let giphy_url = "https://api.giphy.com/v1/gifs/random?api_key=";
        let tag = { 
            message_content.remove(0);
            &message_content.join("%20")[..]
        };
        let rating = get_random_rating();
        let giphy_api_key = &(self.config.read().unwrap().giphy_api_key);
        let request_vector = vec![giphy_url, giphy_api_key, "&tag=", tag, "&rating=", rating.as_str()];
        let request_string = request_vector.join("");
        eprintln!("Request String: {:?}", request_string);
        let gif_response_text: String = reqwest::get(&request_string[..]).unwrap_or_else( |error| {
            eprintln!("Error on request: {:?}", error);
            panic!("Bad Response! {:?}", error);
        }).text().unwrap();
        let gif_response_json: serde_json::value::Value = serde_json::from_str(&gif_response_text).unwrap();
        let gif_json_urls: &serde_json::value::Value = &gif_response_json["data"];
        if let Err(why) = message.channel_id.say(&context.http, gif_json_urls["bitly_gif_url"].as_str().unwrap()) {
            eprintln!("Couldn't send gif: {:?}", why);
        }
    }

    pub fn next_session(&self, context: &Context, message: &Message) {
        let next_session = &(self.config.read().unwrap().next_session);
        let output_vec = vec!["Next session scheduled for: ", next_session];
        let output = output_vec.join("");
        if let Err(why) = message.channel_id.say(&context.http, &output) {
            eprintln!("Couldn't send session details: {:?}", why);
        }
    }

    pub fn commands(&self, context: &Context, message: &Message) {
        let iter = _COMMANDS.into_iter();
        let mut output_vector: Vec<&str> = Vec::new();
        for cmd in iter {
            output_vector.push(cmd);
        }
        if let Err(why) = message.channel_id.say(&context.http, output_vector.join(" ")) {
            eprintln!("Couldn't send commands: {:?}", why);
        }
    }

    pub fn update_session(&self, context: &Context, message: &Message, new_session: &mut Vec<&str>) {
        new_session.remove(0);
        let session_string = new_session.join(" ");
        self.config.write().unwrap().next_session = session_string;
        if let Err(why) = self.config.read().unwrap().save_config() {
             eprintln!("Couldn't save updated configuration: {:?}", why);
         }
        self.next_session(context, message);
    }

}

fn get_random_rating() -> String {
    let ratings_array = ["g", "pg", "pg-13"];
    let rating_index = rand::thread_rng().gen_range(0,3);
    String::from(ratings_array[rating_index])

}