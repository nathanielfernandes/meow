use std::{borrow::Cow, io::Cursor};

use crate::prelude::*;
use image::RgbaImage;

use canvas::prelude::*;
use rand::seq::SliceRandom;

use crate::prelude::{CANVAS_OPTIONS, MEOW_SCRIPT};

pub fn render_text(
    text: &str,
    signed: Option<&str>,
    _identifier: &str,
    font: &str,
    attach: Option<RgbaImage>,
    sentiment: f32,
) -> Option<RgbaImage> {
    let (text, emojis) = imagetext::emoji::parse::parse_out_emojis(&text, true, true);

    let font = FontDB::superfont(&[font, "notojp", "notosc", "nototc"])?;

    let s = scale(30.0);
    let lines = imagetext::wrap::text_wrap(
        &text,
        500,
        &font,
        s,
        WrapStyle::Character,
        imagetext::measure::text_width_with_emojis,
    );

    let (mut w, mut h) = parsed_text_size_multiline_with_emojis(&lines, &font, s, 1.0);

    w = w.max(256) + 50;
    h = h + 50 + 20;

    if h < 30 {
        error!("Height is less than 30");
        return None;
    }

    if attach.is_some() {
        h += 256;
    }

    let mut img = RgbaImage::new(w as u32, h as u32);

    let Ok(mut canvas) = Canvas::new(MEOW_SCRIPT.clone(), &mut img, CANVAS_OPTIONS) else {
        error!("Failed to create canvas");
        return None;
    };

    canvas.add_variable("attachment", attach.is_some());
    canvas.add_variable("sentiment", sentiment);

    if let Some(attach) = &attach {
        canvas.add_image("image", attach);
    }

    if let Err(e) = canvas.run() {
        error!("Failed to run canvas: {}", e.0);
        return None;
    }

    if let Err(_) = draw_parsed_text_multiline_with_emojis(
        &mut img,
        &imagetext::drawing::paint::WHITE,
        Outline::None,
        25.0,
        25.0,
        0.0,
        0.0,
        500.0,
        s,
        &font,
        DefaultEmojiResolver::<true>,
        &lines,
        &emojis,
        &mut 0,
        1.0,
        TextAlign::Left,
    ) {
        error!("Failed to draw text");
        return None;
    }

    if let Some(sign) = signed {
        let _ = draw_text_anchored(
            &mut img,
            &imagetext::drawing::paint::WHITE,
            Outline::None,
            6.0,
            h as f32 - 6.0,
            0.0,
            1.0,
            scale(17.0),
            &font,
            sign,
        );
    }

    let Some(font) = FontDB::query_with_emoji(
        "ggsans-bold",
        EmojiOptions {
            parse_shortcodes: false,
            source: EmojiSource::Apple,
            ..Default::default()
        },
    ) else {
        return Some(img);
    };

    if let Err(_) = draw_text_anchored_with_emojis(
        &mut img,
        &imagetext::drawing::paint::WHITE,
        Outline::None,
        w as f32 - 3.0,
        h as f32 - 3.0,
        1.0,
        1.0,
        scale(17.0),
        &font,
        DefaultEmojiResolver::<false>,
        "😇                                    😈",
    ) {
        return None;
    }

    // if let Err(_) = draw_text_anchored_with_emojis(
    //     &mut img,
    //     &imagetext::drawing::paint::WHITE,
    //     Outline::None,
    //     w as f32 - 3.0,
    //     h as f32 - 3.0,
    //     1.0,
    //     1.0,
    //     scale(17.0),
    //     &font,
    //     DefaultEmojiResolver::<false>,
    //     identifier,
    // ) {
    //     return None;
    // }

    Some(img)
}

pub fn attachment(img: &RgbaImage) -> Result<serenity::AttachmentType, CharmError> {
    let mut buf = Vec::new();
    let mut writer = Cursor::new(&mut buf);

    img.write_to(&mut writer, image::ImageOutputFormat::Png)
        .map_err(|_| CharmError::MeowError("Failed to encode image".to_string()))?;

    Ok(serenity::AttachmentType::Bytes {
        data: Cow::Owned(buf),
        filename: "meow.png".to_string(),
    })
}

pub async fn attachment_to_image(attachment: &Option<serenity::Attachment>) -> Option<RgbaImage> {
    let attachment = attachment.as_ref()?;
    let file = attachment.download().await.ok()?;
    let img = image::load_from_memory(&file).ok()?.to_rgba8();
    Some(img)
}

static EMOJIS: [char; 16] = [
    '😭', '🍆', '🍑', '😔', '💀', '🤓', '🙀', '🥺', '😎', '😅', '🙈', '😠', '🥰', '🥶', '😳', '🤩',
];

pub fn random_identifier(len: usize) -> String {
    // pick unique numbers from 0..16
    let mut rng = rand::thread_rng();
    let mut choices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    choices.shuffle(&mut rng);

    let mut identifier = String::with_capacity(len);

    for i in 0..len {
        identifier.push(EMOJIS[choices[i]]);
    }

    identifier
}
