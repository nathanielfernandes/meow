#![feature(try_blocks)]

pub mod appstate;
pub mod commands;
pub mod generate;
pub mod prelude;
pub mod utils;

use canvas::prelude::*;
use prelude::*;

use crate::commands::*;

#[tokio::main]
async fn main() {
    // load all lazy vars
    let _ = *SECRET;
    let _ = *CHANNEL_ID;
    let _ = *GUILD_ID;
    let _ = *BOT_ID;

    // setup fontdb and emoji source
    FontDB::load_from_dir("assets/fonts");
    let mut opts = EmojiOptions::dir("assets/twemojis");
    opts.parse_discord_emojis = true;
    FontDB::set_default_emoji_options(opts);

    info!(
        "Loaded {} fonts from assets/fonts ✅",
        FontDB::inner()
            .read()
            .expect("Failed to read font database")
            .len()
    );

    // setup logging and dotenv
    dotenvy::dotenv().ok();
    pretty_env_logger::init_timed();

    info!("Logger initialized");

    // setup framework
    let framework = poise::Framework::builder()
        .token(std::env::var("TOKEN").expect("DISCORD_TOKEN not set"))
        .options(poise::FrameworkOptions {
            commands: vec![reply(), meow(), poll()],
            ..Default::default()
        })
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS,
        )
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Established connection to discord");

                poise::builtins::register_in_guild(
                    &ctx.http,
                    &framework.options().commands,
                    *GUILD_ID,
                )
                .await?;

                info!("Registered guilds");

                info!("{} is ready ✅", ready.user.name);

                Ok(appstate::AppState::new().await)
            })
        });

    // run framework
    framework.run().await.expect("Failed to start framework");
}
