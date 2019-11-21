extern crate serenity;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod bot_commands;
mod bot;
use bot::configuration;


fn main() {
    let the_bot = bot::bot::new();
    let mut client = Client::new(the_bot.get_bot_token(), Handler{ the_bot }).expect("Error Creating Client!");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}

struct Handler {
    the_bot: bot::bot,
}

impl EventHandler for Handler {

    fn message(&self, context: Context, message: Message) {
        eprintln!("{:?}", message);
        let mut message_content: Vec<&str> = message.content.split(' ').collect();
        eprintln!("Message Content First Piece: {}", message_content[0]);
        match message_content[0] {
            "!test" => self.the_bot.test(&context, &message),
            "!commands" => self.the_bot.commands(&context, &message),
            "!add" => self.the_bot.add(&context, &message, &message_content),
            "!gif" => self.the_bot.gif(&context, &message, &mut message_content),
            "!nextsession" => self.the_bot.next_session(&context, &message),
            "!setnextsession" => { 
                let saved = self.update_session(&mut message_content);
                self.the_bot.next_session(&context, &message);
            },
            _ => {eprintln!("No Action to take.")}
        }
    }
}

impl Handler {
    fn update_session(&self, new_session: &mut Vec<&str>) -> std::io::Result<()>{
        /*new_session.remove(0);
        let session_string = new_session.join(" ");
        self.config.write().unwrap().next_session = session_string;
        self.config.read().unwrap().save_config()*/
        Ok(())
    } 
}