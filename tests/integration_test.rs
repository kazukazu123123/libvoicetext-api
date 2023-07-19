use core::panic;
use std::{env, time::Duration};
extern crate libvoicetext_api;

#[tokio::test]
async fn api_access() {
    let Ok(api_key) = env::var("VOICETEXT_API") else {panic!("The environment variable 'VOICETEXT_API' is not set. Please set it up.")};
    assert!(libvoicetext_api::get_audio_data(
        api_key,
        libvoicetext_api::ApiOptions {
            text: "test".to_owned(),
            ..Default::default()
        },
        Duration::from_secs(1),
    )
    .await.is_ok())
}
