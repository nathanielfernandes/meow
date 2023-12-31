use canvas::canvas::CanvasOptions;
use canvas::prelude::Script;
pub use futures;
pub use futures::future;
use image::RgbaImage;
pub use log::{debug, error, info, trace, warn};

use once_cell::sync::Lazy;
pub use poise;
pub use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;

use crate::appstate::AppState;

#[derive(Debug, thiserror::Error)]
pub enum CharmError {
    #[error("<Serenity: {0}>")]
    SerenityError(#[from] serenity::Error),

    #[error("{0}")]
    MeowError(String),
}

pub type Data = &'static AppState;

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

pub const CANVAS_OPTIONS: CanvasOptions = CanvasOptions {
    max_runtime: std::time::Duration::MAX,
    stack_size: 32,
    call_stack_size: 16,
    string_max_size: 128,
    array_max_size: 128,
    image_storage_size: 200000000,
};

pub fn leak<T>(t: T) -> &'static T {
    Box::leak(Box::new(t))
}

pub type Transaction<'a> = sqlx::Transaction<'a, sqlx::Postgres>;
pub type Pool = sqlx::PgPool;

const MEOW: &'static str = r#"

let w, h = @Dimensions()

with @PushRelative(0, 10, w - 10, h - 10) {

let w, h = @Dimensions()


let bottom_lip = 26

let bgcolor = #1f1f1f
let fgcolor = #2e2e2e

// if sentiment > 0.9 {
//     bgcolor = #700101
//     fgcolor = #b00b0b
// }

let w, h = @Dimensions()
let bottom_lip = 26

@DrawRoundedRectangle(0,0 * 2, w, h, 10)
@SetColor(bgcolor)
@Fill()

@DrawRoundedRectangle(5, 5, w - 10, h - bottom_lip, 5)
@SetColor(fgcolor)
@FillPreserve()

@Scale(0.5, 0.5)
@SetPattern(snow, "repeat", 0.4)
@Identity()
@SetBlendMode("color_burn")
@Fill()
@SetBlendMode("default")

// let bw, bh = (100, 10)
// let bx, by = (w - bw - 20, h - bh - 6)
// @DrawRoundedRectangle(bx, by, bw, bh, bh)

// @SetLinearGradient((bx, by), (bx + bw, by + bh), "pad", [
//     (0.0, #3bb0ff),
//     (0.2, #3bb0ff),
//     //(0.5, #3bff52),
//     (0.7, #a03bff),
//     (1.0, #a03bff)
// ])
// @Fill()

// @DrawCircle(bx + bw * @clamp(sentiment, 0.0, 1.0), by + bh / 2, 7)
// @SetColor(#ffffff)
// @Fill()

if attachment {
    let s = 256
    let z = h - s - bottom_lip
    @DrawRoundedRectangle(10, z, w - 20, s, 5)
    @ClipPreserve()
    
    @SetFilter("good")
    let img = @Blur(image, 20.0)
    @DrawImageCovered(img, 0, z, w, s)
    
    @SetColor(#00000056)
    @Fill()

    @ResetClip()
    @DrawImageContained(image, 10, z, w - 20, s)
}

}

@DrawImageAnchoredSized(santa, w - 18, 0, 0.5, 0.0, 30, 40)

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

const POLL: &'static str = r#"
let w, h = @Dimensions()
let bottom_lip = 26

let bgcolor = #1f1f1f
let fgcolor = #2e2e2e

@DrawRoundedRectangle(0,0 * 2, w, h, 10)
@SetColor(bgcolor)
@Fill()

@DrawRoundedRectangle(5, 5, w - 10, h - bottom_lip, 5)
@SetColor(fgcolor)
@Fill()

let fontsize = 30
let pad = 20
let bh = 20
let x = pad
let y = offset + pad
let r = 4

@SetFont("ggsans-semibold notojp notosc nototc notob")
@SetFontSize(fontsize)

let i = 1
for name, _ in choices {
    @DrawString(@truncate(name, 45, "..."), x, y)
    y += fontsize * 2
    i += 1
}

@SetColor(#fff)
@Fill()

y = pad + offset
for _, _ in choices {
    @DrawRoundedRectangle(x, y + fontsize, w - pad * 2, bh, r)
    y += fontsize * 2
}

@SetColor(bgcolor)
@Fill()


y = pad + offset
for _, votes in choices {
    let percent = @clamp(@float(votes) / @max(total, 1), 0.0, 1.0)
    @DrawRoundedRectangle(x, y + fontsize, (w - pad * 2) * percent, bh, r)
    y += fontsize * 2
}


@SetLinearGradient((0.0, 0.0), (w, h), "pad", [
    (0.0, #ba71ff),
    (1.0, #494cff),
])
@Fill()
"#;

pub const POLL_SCRIPT: Lazy<Script> = Lazy::new(|| {
    let Ok(script) = canvas::prelude::build_script(
        "poll",
        POLL,
        &mut canvas::prelude::DefaultIncludeResolver::default(),
    ) else {
        panic!("Failed to build poll script");
    };
    script
});

pub const SNOW_IMAGE: Lazy<RgbaImage> = Lazy::new(|| {
    image::open("./assets/images/snow.png")
        .expect("Failed to load snow image")
        .to_rgba8()
});

pub const SANTA_HAT_IMAGE: Lazy<RgbaImage> = Lazy::new(|| {
    image::open("./assets/images/santa.png")
        .expect("Failed to load santa image")
        .to_rgba8()
});
