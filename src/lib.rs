use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub mod bot;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command =
            Command::create_global_command(&ctx.http, bot::commands::qrcode::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "qrcode" => Some(bot::commands::qrcode::run(&command).await),
                "ping" => Some(bot::commands::qrcode::run(&command).await),
                _ => Some(CreateInteractionResponseMessage::new().content("Not implemented")),
            };

            if let Some(content) = content {
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

pub async fn run_discord_bot(token: &str) {

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}