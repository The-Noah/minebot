use std::env;

pub async fn get_players() -> Vec<String> {
  let connection = async_minecraft_ping::ConnectionConfig::build(env::var("MINECRAFT_IP").expect("Expected MINECRAFT_IP in the environment"))
    .connect()
    .await
    .unwrap();

  let status = connection.status().await.unwrap().status;

  let mut players = Vec::new();

  if status.players.online > 0 && status.players.sample.is_some() {
    for player in status.players.sample.unwrap() {
      players.push(player.name);
    }
  }

  players
}
