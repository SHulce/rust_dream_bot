extern crate serenity;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::sync::RwLock;

mod bot_commands;
mod configuration;

fn main() {
    let config: configuration::Config = configuration::initialize_config();
    let mut client = Client::new(config.bot_token.clone(), Handler{ config: RwLock::new(config) }).expect("Error Creating Client!");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}

struct Handler {
    config: RwLock<configuration::Config>,
}

impl EventHandler for Handler {

    fn message(&self, context: Context, message: Message) {
        eprintln!("{:?}", message);
        let mut message_content: Vec<&str> = message.content.split(' ').collect();
        eprintln!("Message Content First Piece: {}", message_content[0]);
        match message_content[0] {
            "!test" => bot_commands::test(&context, &message),
            "!commands" => bot_commands::commands(&context, &message),
            "!add" => bot_commands::add(&context, &message, &message_content),
            "!gif" => bot_commands::gif(&context, &message, &mut message_content, &self.config.read().unwrap().giphy_api_key),
            "!nextsession" => bot_commands::next_session(&context, &message, &self.config.read().unwrap().next_session),
            "!setnextsession" => { 
                let saved = self.update_session(&mut message_content);
                bot_commands::next_session(&context, &message, &self.config.read().unwrap().next_session);
            },
            _ => {eprintln!("No Action to take.")}
        }
    }
}

impl Handler {
    fn update_session(&self, new_session: &mut Vec<&str>) -> std::io::Result<()>{
        new_session.remove(0);
        let session_string = new_session.join(" ");
        self.config.write().unwrap().next_session = session_string;
        self.config.read().unwrap().save_config()
    } 
}