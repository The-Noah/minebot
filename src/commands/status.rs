use serenity::builder::CreateCommand;

use crate::minecraft;

pub async fn run() -> String {
  let players = minecraft::get_players().await;

  let mut response = format!("Server is online with {} player{}", players.len(), if players.len() == 1 { "" } else { "s" });

  if players.len() > 0 {
    response.push_str("\n# Players");

    for player in players {
      response.push_str(&format!("\n- {}", player));
    }
  }

  response
}

pub fn register() -> CreateCommand {
  CreateCommand::new("status").description("Get server status and player list")
}
