extern crate serenity;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod bot_commands;
mod configuration;

fn main() {
    let config: configuration::Config = configuration::initialize_config();
    let mut client = Client::new(config.bot_token.clone(), Handler{ config }).expect("Error Creating Client!");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}

struct Handler {
    config: configuration::Config, // Cell/RefCell/Box
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
            "!gif" => bot_commands::gif(&context, &message, &mut message_content, &self.config.giphy_api_key),
            "!nextsession" => {
                let saved = self.update_session(&mut message_content);
                bot_commands::next_session(&context, &message, &self.config.next_session) },
            "!setnextsession" => bot_commands::set_next_session(&context, &message, &self.config),
            _ => {eprintln!("No Action to take.")}
        }
    }
}

impl Handler {
    fn update_session(&mut self, new_session: &mut Vec<&str>) -> std::io::Result<()>{
        new_session.remove(0);
        let session_string = new_session.join(" ");
        self.config.next_session = session_string;
        self.config.save_config()
    } 
}