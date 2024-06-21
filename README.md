# minebot

A simple Discord bot to interact with your Minecraft server.

## Installation

1. Clone the repository

```bash
git clone https://github.com/The-Noah/minebot.git
```

2. Build bot

```bash
cd minebot
cargo build --release
```

3. Create a `.env` file in the root of the project and add the following:

```env
DISCORD_TOKEN=your_discord_bot_token
GUILD_ID=your_discord_guild_id
MINECRAFT_IP=your_minecraft_server_ip
```

4. Run the bot

```bash
./target/release/minebot
```

## Usage

The bot will automatically register slash commands when it starts. You can use the following commands:

- `/ping` - Check if the bot is online
- `/status` - Get the status of the Minecraft server with player info

The bot will also automatically update its status to show the number of players online.

## License

[MIT](LICENSE)
