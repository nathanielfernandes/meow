use poise::Modal;

use crate::{db::sentiment::Sentiment, generate::*, prelude::*};

#[derive(Debug, poise::Modal)]
pub struct ReplyModal {
    #[name = "Reply"]
    #[placeholder = "reply..."]
    #[paragraph]
    reply: String,
}

#[poise::command(context_menu_command = "reply")]
pub async fn reply(
    ctx: ApplicationContext<'_>,
    #[description = "meow to reply to"] msg: serenity::Message,
) -> CommandResult {
    // check if message author id is same as the bot Id
    if msg.author.id == *BOT_ID {
        return Ok(());
    }

    let Some(data) = ReplyModal::execute(ctx).await? else {
        return Ok(());
    };

    let identifier = ""; // random_identifier(5);

    let sentiment = Sentiment::analyze(&data.reply, &ctx.data().client)
        .await
        .linear() as f32;

    let Some(img) = render_text(
        &data.reply,
        None,
        &identifier,
        "ggsans-bold",
        None,
        sentiment,
    ) else {
        return Ok(());
    };

    let attachment = attachment(&img)?;

    msg.channel_id
        .send_message(ctx.serenity_context(), |m| {
            m.add_file(attachment).reference_message(&msg)
        })
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn meow(
    ctx: Context<'_>,
    #[description = "message"] message: String,
    #[description = "image"] image: Option<serenity::Attachment>,
    #[description = "anonymous"] anonymous: Option<bool>,
    #[description = "font name"] font: Option<Font>,
) -> CommandResult {
    let msg = ctx
        .send(|reply| {
            reply
                .content("Sending... 😈")
                .ephemeral(true)
                .allowed_mentions(|allowed| allowed.empty_parse())
        })
        .await?;

    let signed = if anonymous.unwrap_or(true) {
        None
    } else {
        Some(ctx.author().name.as_str())
    };

    let identifier = ""; // random_identifier(5);

    let sentiment = Sentiment::analyze(&message, &ctx.data().client)
        .await
        .linear() as f32;

    let attach = attachment_to_image(&image).await;
    let img = render_text(
        &message,
        signed,
        &identifier,
        font.unwrap_or(Font::Discord).str(),
        attach,
        sentiment,
    )
    .ok_or(CharmError::MeowError("Failed to render text".to_string()))?;

    let attachment = attachment(&img)?;

    (*CHANNEL_ID)
        .send_files(ctx.serenity_context(), vec![attachment], |m| m)
        .await?;

    msg.edit(ctx, |m| {
        m.content("Sent! 😈")
            .allowed_mentions(|allowed| allowed.empty_parse())
    })
    .await?;

    Ok(())
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum Font {
    #[name = "arialbold"]
    ArialBold,
    #[name = "calibribold"]
    CalibriBold,
    #[name = "comicsans"]
    ComicSans,
    #[name = "impact"]
    Impact,
    #[name = "discord"]
    Discord,
    #[name = "coolvetica"]
    Coolvetica,
    #[name = "times"]
    Times,
}

impl Font {
    pub fn str(&self) -> &'static str {
        match self {
            Font::ArialBold => "arialbold",
            Font::CalibriBold => "calibribold",
            Font::ComicSans => "comicsans",
            Font::Impact => "impact",
            Font::Discord => "ggsans-semibold",
            Font::Coolvetica => "coolvetica",
            Font::Times => "times",
        }
    }
}
