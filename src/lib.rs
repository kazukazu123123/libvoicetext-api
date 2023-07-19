use serde::{Deserialize, Serialize};
use serde_urlencoded;
use std::str::FromStr;
use std::time::Duration;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Speaker {
    Show,
    Haruka,
    Hikari,
    Takeru,
    Santa,
    Bear,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Wav,
    Ogg,
    Mp3,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    Happiness,
    Anger,
    Sadness,
}

#[derive(Deserialize, Serialize)]
pub struct ApiOptions {
    pub text: String,
    pub speaker: Speaker,
    pub format: Option<AudioFormat>,
    pub emotion: Option<Emotion>,
    pub emotion_level: Option<u8>,
    pub pitch: Option<u16>,
    pub speed: Option<u16>,
    pub volume: Option<u16>,
}

impl Default for ApiOptions {
    fn default() -> Self {
        Self {
            text: Default::default(),
            speaker: Speaker::Show,
            format: None,
            emotion: None,
            emotion_level: None,
            pitch: None,
            speed: None,
            volume: None,
        }
    }
}

impl FromStr for ApiOptions {
    type Err = serde_urlencoded::de::Error;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str(str)
    }
}

pub async fn get_audio_data(
    api_key: String,
    options: ApiOptions,
    timeout: Duration,
) -> reqwest::Result<Vec<u8>> {
    let query_paramator = serde_urlencoded::to_string(options).unwrap();

    const API_ENDPOINT: &str = "https://api.voicetext.jp/v1/tts";

    let client = reqwest::Client::builder().timeout(timeout).build()?;

    let response = client
        .post(API_ENDPOINT)
        .basic_auth(api_key, None::<&str>)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query_paramator)
        .send()
        .await?;

    let audio_data = response.bytes().await?;

    Ok(audio_data.to_vec())
}
