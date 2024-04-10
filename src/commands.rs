use std::collections::HashMap;

use poise::Modal;

use crate::{generate::*, prelude::*, utils::sentiment::Sentiment};

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

    info!(
        "{}#{} sent: {}",
        ctx.author().name,
        ctx.author().discriminator,
        data.reply
    );

    let identifier = ""; // random_identifier(5);

    let sentiment = Sentiment::analyze(&data.reply, &ctx.data().client)
        .await
        .linear() as f32;

    let Some(img) = render_text(
        &data.reply,
        None,
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
    #[description = "role"] role: Option<serenity::Role>,
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

    let role_stuff = match role {
        Some(role) => {
            let guild = role.guild_id;
            if !ctx
                .author()
                .has_role(ctx.serenity_context(), guild, role.id)
                .await?
            {
                return Err(CharmError::MeowError(
                    "You don't have the required role".to_string(),
                ));
            }

            let role_mention = format!("@{}", role.name);
            let role_color: u32 = role.colour.0;
            let role_color: [u8; 4] = [
                ((role_color >> 16) & 0xFF) as u8,
                ((role_color >> 8) & 0xFF) as u8,
                (role_color & 0xFF) as u8,
                255,
            ];
            Some((role_mention, role_color))
        }
        None => None,
    };

    update_db(ctx.author().id.0, &message);

    let attach = attachment_to_image(&image).await;
    let img = render(&message, signed, role_stuff, attach)
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

#[poise::command(slash_command)]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "question"] question: String,
    #[description = "choice 1"] choice1: String,
    #[description = "choice 2"] choice2: String,
    #[description = "choice 3"] choice3: Option<String>,
    #[description = "choice 4"] choice4: Option<String>,
    #[description = "choice 5"] choice5: Option<String>,
    #[description = "anonymous"] anonymous: Option<bool>,
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

    let mut choices = vec![(choice1, 0), (choice2, 0)];
    if let Some(choice3) = choice3 {
        choices.push((choice3, 0));
    }
    if let Some(choice4) = choice4 {
        choices.push((choice4, 0));
    }
    if let Some(choice5) = choice5 {
        choices.push((choice5, 0));
    }

    let img = render_poll(&question, &choices, 0, signed)
        .ok_or(CharmError::MeowError("Failed to render poll".to_string()))?;

    let atch = attachment(&img)?;

    let poll_msg = (*CHANNEL_ID)
        .send_files(ctx.serenity_context(), vec![atch], |m| {
            m.components(|c| {
                c.create_action_row(|ar| {
                    for (i, _) in choices.iter().enumerate() {
                        ar.create_button(|b| {
                            b.label(format!("Choice {}", i + 1))
                                .style(serenity::ButtonStyle::Primary)
                                .custom_id(format!("poll_{}", i))
                        });
                    }
                    ar
                })
            })
        })
        .await?;

    msg.edit(ctx, |m| {
        m.content("Sent! 😈")
            .allowed_mentions(|allowed| allowed.empty_parse())
    })
    .await?;

    let mut voters: HashMap<u64, usize> = HashMap::new();
    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .message_id(poll_msg.id)
        .channel_id(*CHANNEL_ID)
        .filter(move |mci| mci.data.custom_id.starts_with("poll"))
        .await
    {
        // defer the interaction
        mci.defer(ctx).await?;

        let user_id = mci.user.id.0;
        let choice = mci
            .data
            .custom_id
            .split('_')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .expect("Failed to parse choice");

        if let Some(prev_choice) = voters.insert(user_id, choice) {
            choices[prev_choice].1 -= 1;
        }

        choices[choice].1 += 1;

        let img = render_poll(&question, &choices, voters.len() as i32, signed)
            .ok_or(CharmError::MeowError("Failed to render poll".to_string()))?;

        let atch = attachment(&img)?;

        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| m.remove_all_attachments().attachment(atch))
            .await?;
    }

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

fn update_db(user: u64, msg: &str) {
    todo!("update db")
}
