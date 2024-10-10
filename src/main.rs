mod yt_search;
mod yt_dlp;

use yt_search::{get_top_result, web_scrape};
use yt_dlp::download_audio;
use std::env::var;

#[tokio::main]
async fn main() {
    let programdir = match var("APPDATA") {
        Ok(appdata) => format!("{appdata}\\rust_music_player\\"),
        Err(_) => panic!("Couldn't find APPDATA env var.")
    };
    let url: String = String::from("");
    let html = web_scrape(&url).await.unwrap();
    let _ = match get_top_result(&html) {
        Some(url) => download_audio(&url, &programdir),
        None => Err(String::new())
    };
}
