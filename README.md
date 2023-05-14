# Simple Price Bot using Coingecko Crate

Required env variables:

- `DISCORD_TOKEN`: [Discord](https://discord.com/developers/applications/) bot token
- `TOKEN_ID`: Token ID can be retrieved on [/coins/list](https://www.coingecko.com/en/api/documentation)
- `GUILD_ID`: ID of the discord channel

## Important crates used:
- `serenity`: Discord API in Rust
  - https://docs.shuttle.rs/resources/shuttle-secrets
- `currency_rs`: Formats float to currency
  - https://docs.rs/currency_rs/latest/currency_rs/#
- `shuttle`: Rust server deployment
  - https://docs.rs/shuttle-service/latest/shuttle_service/
- `coingecko`
  - https://docs.rs/coingecko/latest/coingecko/struct.CoinGeckoClient.html#implementations

## Deployment
```
# Initialize `shuttle` configurations. Projects will be hosted at 
# ${project_name}.shuttleapp.rs
cargo shuttle init --serenity

# Existing serenity projects, you can start it with:
cargo shuttle project start --name=${project_name}

# Run the program locally 
cargo shuttle run --name=${project_name}

# Deploy and run the program 
cargo shuttle deploy --name=${project_name}
```

