use std::env;

use dotenv::dotenv;

use serenity::{
  all::{CreateInteractionResponse, CreateInteractionResponseMessage},
  async_trait,
  model::{application::Interaction, gateway::Ready, id::GuildId},
  prelude::*,
};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let guild_id = GuildId::new(
      env::var("GUILD_ID")
        .expect("Expected GUILD_ID in the environment")
        .parse()
        .expect("GUILD_ID must be an integer"),
    );

    let commands = guild_id.set_commands(&ctx.http, vec![commands::ping::register()]).await;

    println!("Commands: {commands:#?}");
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Command: {command:#?}");

      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        _ => Some("Unknown command".to_string()),
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Error sending response: {:?}", why);
        }
      }
    }
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

  let intents = GatewayIntents::GUILD_MESSAGES;

  let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
