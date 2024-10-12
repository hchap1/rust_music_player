mod yt_search;
mod yt_dlp;

use yt_search::{get_top_result, web_scrape};
use yt_dlp::{open_folder, download_audio};
use std::fs::create_dir;
use std::env::{args, Args, var};
use std::collections::HashSet;

fn parse_args(arguments: Args) -> (Option<String>, Vec<String>, HashSet<String>) {
    let mut args_vec: Vec<String> = vec![];
    let mut flags: HashSet<String> = HashSet::new();
    for arg in arguments {
        if arg.starts_with("-") {
            flags.insert(arg.to_string());
        }
        args_vec.push(arg.to_string());
    }
    let command: Option<String> = args_vec.get(1).cloned();
    (command, args_vec, flags)
}

#[tokio::main]
async fn main() {
    let help = r#"
    rust_music_player <COMMAND> <FLAGS/ARGUMENTS>
                        |
     -------------------
    |
    download <arg>  -u: Direct URL (otherwise search)
    play     <SONG> -c: Shuffle afterwards
    shuffle
    stop     <arg>  -t: Stop after <arg> minutes
    pause
    resume
    skip
    queue
    open
    "#;

    let programdir = match var("APPDATA") {
        Ok(appdata) => format!("{appdata}\\rust_music_player\\"),
        Err(_) => panic!("Couldn't find APPDATA env var.")
    };
    let arguments: Args = args();
    let (command, args_vec, flags) = parse_args(arguments);

    if let None = command {
        println!("{help}");
        return;
    }

    let command: String = command.unwrap();
    let _ = create_dir(&programdir);
    
    match command.as_str() {
        "open" => {
            open_folder(&programdir);
            return;
        }
        "download" => {
            let url: String = match flags.contains(&String::from("-u")) {
                true => match args_vec.get(2) {
                    Some(address) => address.to_string(),
                    None => panic!("Expected download <URL> -u")
                },
                false => {
                    let query: String = match args_vec.get(2) {
                        Some(search) => search.to_string(),
                        None => panic!("Expected download <QUERY>")
                    };
                    let html = web_scrape(&query).await.unwrap();
                    match get_top_result(&html) {
                        Some(result) => result,
                        None => panic!("No results found for query {query}")
                    }
                }
            };
            match download_audio(&url, &programdir) {
                Ok(_) => println!("Successfully downloaded {url}"),
                Err(e) => panic!("{e}")
            }
            return;
        }
        "play" => {

        }
        _ => eprintln!("Unknown command: {command}")
    }
}
