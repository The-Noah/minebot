use serenity::all::{CreateCommandOption, Permissions};
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::minecraft;

pub async fn run<'a>(options: &'a [ResolvedOption<'a>]) -> String {
  if let Some(ResolvedOption {
    value: ResolvedValue::String(message),
    ..
  }) = options.first()
  {
    minecraft::rcon_command(&format!("say {}", message)).await;

    format!("Sent message: `{}`", message)
  } else {
    "No message provided".to_string()
  }
}

pub fn register() -> CreateCommand {
  CreateCommand::new("say")
    .description("Send a message to the server")
    .default_member_permissions(Permissions::ADMINISTRATOR)
    .add_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "message", "Message").required(true))
}
