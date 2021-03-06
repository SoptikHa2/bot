# Elin Bot

A Discord bot written using the Serenity framework in Rust. With a focus on proper logging and zero embeds, this general purpose bot was made to fill in the gap in the market. Everyone is encouraged to run their own.

## Manual setup
1. Clone the repository.
2. `cargo build` to get the executable.
3. Open `config.toml` and insert your Discord bot token.
4. `cargo run` to run the bot.

## Docker-compose setup
```
version: "3"

services:
  bot:
    image: "elinvynia/bot:latest"
    environment:
      - DISCORD_TOKEN=YourTokenHere
    volumes:
      - .path/to/local/db.sqlite3:/db.sqlite3
```
