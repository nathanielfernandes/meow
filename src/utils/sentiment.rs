use serde::Deserialize;

use crate::prelude::SECRET;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Sentiment {
    pub positive: f64,
    pub negative: f64,
    pub neutral: f64,
}

impl Sentiment {
    pub async fn analyze(text: &str, client: &reqwest::Client) -> Self {
        // https://aware.ncp.nathanferns.xyz/sentiment/<text>
        let url = format!(
            "https://aware.ncp.nathanferns.xyz/sentiment/{}",
            urlencoding::encode(text)
        );

        let Ok(res) = client
            .get(&url)
            .header("SECRET", SECRET.as_str())
            .send()
            .await
        else {
            return Self::default();
        };

        let Ok(res) = res.json::<Sentiment>().await else {
            return Self::default();
        };

        res
    }

    pub fn linear(&self) -> f64 {
        let total = self.positive + self.negative + self.neutral;
        if total == 0.0 {
            return 0.0;
        }

        // scalar value between -1 and 1 where -1 is positive, 0 is neutral, and 1 is negative
        let s = (self.negative - self.positive) / total;

        // return clamp s to be from 0.0 to 1.0
        (s + 1.0) / 2.0
    }

    pub fn new() -> Self {
        Self {
            positive: 0.0,
            negative: 0.0,
            neutral: 0.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserSentiment {
    pub positive: f64,
    pub negative: f64,
    pub neutral: f64,
    pub records: f64,
}

impl UserSentiment {
    pub fn new() -> Self {
        Self {
            positive: 0.0,
            negative: 0.0,
            neutral: 0.0,
            records: 0.0,
        }
    }

    pub fn add(&mut self, sentiment: &Sentiment) {
        self.positive += sentiment.positive;
        self.negative += sentiment.negative;
        self.neutral += sentiment.neutral;
        self.records += 1.0;
    }

    pub fn positive(&self) -> f64 {
        self.positive / self.records
    }

    pub fn negative(&self) -> f64 {
        self.negative / self.records
    }

    pub fn neutral(&self) -> f64 {
        self.neutral / self.records
    }

    pub fn total(&self) -> f64 {
        self.positive + self.negative + self.neutral
    }

    pub fn total_normalized(&self) -> f64 {
        self.total() / self.records
    }
}
