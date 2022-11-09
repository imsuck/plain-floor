use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping, say)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("^"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Discord token not found.");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong! 🏓").await?;

    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(ch) = msg.channel(ctx).await?.guild() {
        ch.send_message(ctx, |m| m.content(&msg.content.split_at(5).1))
            .await?;
    }

    Ok(())
}
