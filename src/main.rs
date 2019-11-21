extern crate serenity;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod bot;

fn main() {
    let the_bot = bot::Bot::new();
    let mut client = Client::new(the_bot.get_bot_token(), Handler{ the_bot }).expect("Error Creating Client!");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }

}

struct Handler {
    the_bot: bot::Bot,
}

impl EventHandler for Handler {

    fn message(&self, context: Context, message: Message) {
        eprintln!("{:?}", message);
        let mut message_content: Vec<&str> = message.content.split(' ').collect();
        let command = message_content.remove(0);
        eprintln!("Message Content First Piece: {}", command);
        match command {
            "!test" => self.the_bot.test(&context, &message),
            "!commands" => self.the_bot.commands(&context, &message),
            "!add" => self.the_bot.add(&context, &message, &message_content),
            "!gif" => self.the_bot.gif(&context, &message, &message_content),
            "!nextsession" => self.the_bot.next_session(&context, &message),
            "!setnextsession" => self.the_bot.update_session(&context, &message, &message_content),
            "!dnd" => self.the_bot.dnd_search(&context, &message, &mut message_content),
            _ => {eprintln!("No Action to take.")}
        }
    }
}
