extern crate dotenv;

use std::env;

use dotenv::dotenv;
use log::error;
use serenity::prelude::*;

mod events;

#[tokio::main]
async fn main() {
    dotenv().expect("Could not load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::empty();
    let mut client = Client::builder(token, intents)
        .event_handler(events::Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Failed to start client: {:?}", why);
    }
}