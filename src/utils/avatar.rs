use std::sync::Arc;

use image::RgbaImage;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;

static DEFAULT_AVATAR: Lazy<Arc<RgbaImage>> = Lazy::new(|| {
    let img = image::open("assets/images/discordblue.png").expect("Failed to open default avatar");
    Arc::new(img.to_rgba8())
});

#[derive(Debug, Clone, Deserialize)]
pub struct Avatar {
    pub url: Option<String>,
    #[serde(skip)]
    pub image: Option<Arc<RgbaImage>>,
}
impl Avatar {
    pub async fn resolve(&mut self, client: Client) -> Arc<RgbaImage> {
        if let Some(img) = &self.image {
            return img.clone();
        };

        let Some(url) = &self.url else {
            return DEFAULT_AVATAR.clone();
        };

        let Ok(res) = client.get(&*url).send().await else {
            return DEFAULT_AVATAR.clone();
        };

        let Ok(bytes) = res.bytes().await else {
            return DEFAULT_AVATAR.clone();
        };

        let Ok(img) = image::load_from_memory(&bytes).map(|img| img.to_rgba8()) else {
            return DEFAULT_AVATAR.clone();
        };

        let img = Arc::new(img);

        self.image = Some(img.clone());

        img
    }
}
