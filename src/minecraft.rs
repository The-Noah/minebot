use std::env;

use rcon::Connection;

pub async fn get_players() -> Vec<String> {
  let connection = async_minecraft_ping::ConnectionConfig::build(get_ip()).connect().await.unwrap();

  let status = connection.status().await.unwrap().status;

  let mut players = Vec::new();

  if status.players.online > 0 && status.players.sample.is_some() {
    for player in status.players.sample.unwrap() {
      players.push(player.name);
    }
  }

  players
}

pub async fn rcon_command(command: &str) -> String {
  let mut connection = Connection::builder()
    .enable_minecraft_quirks(true)
    .connect(
      format!("{}:25575", get_ip()),
      &env::var("RCON_PASSWORD").expect("Expected RCON_PASSWORD in the environment"),
    )
    .await
    .unwrap();

  connection.cmd(command).await.unwrap()
}

fn get_ip() -> String {
  env::var("MINECRAFT_IP").expect("Expected MINECRAFT_IP in the environment")
}
