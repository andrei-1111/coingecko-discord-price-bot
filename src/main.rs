use currency_rs::{Currency, CurrencyOpts};
use serenity::model::gateway::Activity;
use serenity::{
    async_trait,
    client::{Client, Context},
    model::gateway::Ready,
    model::id::GuildId,
    model::user::OnlineStatus,
    prelude::*,
};
use shuttle_secrets::SecretStore;

struct Handler {
    guild_id: String,
    token_id: String,
}

fn usd_format(value: f64) -> Currency {
    let otp = CurrencyOpts::new().set_symbol("$").set_precision(2);
    Currency::new_float(value, Some(otp))
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut msg = format!(
            "__Connection Success__\n\n Bot Name: {} \n\n {} guilds connected;\n",
            ready.user.name,
            ready.guilds.len()
        );
        let guild_id = GuildId(self.guild_id.parse().unwrap());
        msg = format!("{}    • {}\n", msg, guild_id);
        println!("{}", msg);

        loop {
            use coingecko::CoinGeckoClient;
            let client = CoinGeckoClient::default();

            let response = client
                .coin(&self.token_id, false, false, true, false, false, false)
                .await
                .unwrap();

            let market_data = response.market_data.unwrap();
            let price = usd_format(market_data.current_price.usd.unwrap_or_default()).format();
            let price_change24_h = market_data.price_change24_h.unwrap_or_default();
            let price_change24_h_format = if price_change24_h > 0.0 {
                format!("+{}", usd_format(price_change24_h).format())
            } else {
                format!("{}", usd_format(price_change24_h).format())
            };
            let percent_change24_h = market_data.price_change_percentage24_h.unwrap_or_default();
            let percent_change24_h = if percent_change24_h > 0.0 {
                format!("+{:.2}%", percent_change24_h)
            } else {
                format!("{:.2}%", percent_change24_h)
            };
            let bot_name = format!("{}° {}", response.symbol.to_uppercase(), &price);

            if let Err(e) = ctx
                .http
                .edit_nickname(*guild_id.as_u64(), Some(&bot_name))
                .await
            {
                println!("Failed to change nickname: {:?}", e);
            } else {
                println!("Nickname successfully changed to {}", bot_name);
            }
            let bot_activity = format!(
                "{} ({})",
                &price_change24_h_format, &percent_change24_h
            );
            if price_change24_h > 0.0 {
                ctx.set_presence(
                    Some(Activity::watching(&bot_activity)),
                    OnlineStatus::Online,
                )
                .await;
            } else {
                ctx.set_presence(
                    Some(Activity::watching(&bot_activity)),
                    OnlineStatus::DoNotDisturb,
                )
                .await;
            }
            // Change the name every 30 secs
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Configure the client with your Discord bot token in the environment.
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN must be set.");
    let guild_id = secret_store.get("GUILD_ID").expect("GUILD_ID must be set.");
    let token_id = secret_store.get("TOKEN_ID").expect("TOKEN_ID must be set.");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler {
            token_id: token_id,
            guild_id: guild_id,
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(client.into())
}
