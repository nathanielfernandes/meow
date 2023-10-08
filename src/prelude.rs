use canvas::canvas::CanvasOptions;
use canvas::prelude::Script;
pub use futures;
pub use futures::future;
pub use log::{debug, error, info, trace, warn};

use once_cell::sync::Lazy;
pub use poise;
pub use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;

#[derive(Debug, thiserror::Error)]
pub enum CharmError {
    #[error("<Serenity: {0}>")]
    SerenityError(#[from] serenity::Error),

    #[error("{0}")]
    MeowError(String),
}

pub type Data = ();

pub type CommandResult = Result<(), CharmError>;
pub type Context<'a> = poise::Context<'a, Data, CharmError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, CharmError>;
pub type Framework = poise::Framework<Data, CharmError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CharmError>;

pub static SECRET: Lazy<String> = Lazy::new(|| std::env::var("SECRET").expect("SECRET not set"));
pub static CHANNEL_ID: Lazy<ChannelId> = Lazy::new(|| {
    let channel_id = std::env::var("CHANNEL_ID").expect("CHANNEL_ID not set");
    channel_id
        .parse::<u64>()
        .expect("CHANNEL_ID is not a valid u64")
        .into()
});
pub static GUILD_ID: Lazy<serenity::GuildId> = Lazy::new(|| {
    let guild_id = std::env::var("GUILD_ID").expect("GUILD_ID not set");
    guild_id
        .parse::<u64>()
        .expect("GUILD_ID is not a valid u64")
        .into()
});
pub static BOT_ID: Lazy<serenity::UserId> = Lazy::new(|| {
    let bot_id = std::env::var("BOT_ID").expect("BOT_ID not set");
    bot_id
        .parse::<u64>()
        .expect("BOT_ID is not a valid u64")
        .into()
});

const MEOW: &'static str = r#"
let w, h = @Dimensions()
let bottom_lip = 26

@DrawRoundedRectangle(0,0 * 2, w, h, 10)
@SetColor(#1f1f1f)
@Fill()

@DrawRoundedRectangle(5, 5, w - 10, h - bottom_lip, 5)
@SetColor(#2e2e2e)
@Fill()
"#;

pub const MEOW_SCRIPT: Lazy<Script> = Lazy::new(|| {
    let Ok(script) = canvas::prelude::build_script(
        "meow",
        MEOW,
        &mut canvas::prelude::DefaultIncludeResolver::default(),
    ) else {
        panic!("Failed to build meow script");
    };
    script
});

pub const CANVAS_OPTIONS: CanvasOptions = CanvasOptions {
    max_runtime: std::time::Duration::MAX,
    stack_size: 32,
    call_stack_size: 16,
    string_max_size: 128,
    array_max_size: 128,
    image_storage_size: 0,
};
