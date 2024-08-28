mod commands;
mod handlers;

extern crate dotenv;
extern crate serenity;

use dotenv::dotenv;
use handlers::run::Handler;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will receive
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
