use serenity::all::CreateCommandOption;
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::minecraft;

pub async fn run<'a>(options: &'a [ResolvedOption<'a>]) -> String {
  let subcommand = options.first();

  if let Some(ResolvedOption {
    name,
    value: ResolvedValue::SubCommand(subcommand_options),
    ..
  }) = subcommand
  {
    let name = name.to_owned();

    if name == "list" {
      let result = minecraft::rcon_command("whitelist list").await;
      let mut players = result.find(':').map(|i| &result[i + 2..]).unwrap_or(result.as_str()).split(", ").collect::<Vec<&str>>();

      players.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

      return players.iter().map(|player| format!("- {}", player.replace("_", "\\_"))).collect::<Vec<String>>().join("\n");
    }

    if let Some(ResolvedOption {
      value: ResolvedValue::String(username),
      ..
    }) = subcommand_options.first()
    {
      match name {
        "add" | "remove" => minecraft::rcon_command(&format!("whitelist {} {}", name, username)).await.replace("_", "\\_"),
        _ => "Unknown subcommand".to_string(),
      }
    } else {
      "No username provided".to_string()
    }
  } else {
    "No subcommand provided".to_string()
  }
}

pub fn register() -> CreateCommand {
  CreateCommand::new("whitelist")
    .description("Manage the whitelist")
    .add_option(
      CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, "add", "Add a player to the whitelist")
        .add_sub_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "username", "Minecraft username").required(true)),
    )
    .add_option(
      CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, "remove", "Remove a player from the whitelist")
        .add_sub_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "username", "Minecraft username").required(true)),
    )
    .add_option(CreateCommandOption::new(
      serenity::all::CommandOptionType::SubCommand,
      "list",
      "List all players on the whitelist",
    ))
}
