mod yt_search;
mod yt_dlp;
mod download;
mod music;
mod process;

use download::DownloadTask;
use yt_dlp::open_folder;
use std::fs::create_dir;
use std::env::{args, Args, var};
use std::collections::HashSet;
use process::run_server;
use music::Player;

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
                        |          |
     -------------------           |
    |                              |
    download <arg>  -u: Direct URL (otherwise search) -f: Read songs in from file
    play     <song> -c: Shuffle afterwards
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

    let is_process = flags.contains(&String::from("-process"));

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
            let download_task: DownloadTask = match DownloadTask::from_args(args_vec, flags, programdir) {
                Ok(task) => task,
                Err(e) => {
                    eprintln!("{e:?}");
                    return;
                }
            };
            if let Err(errors) = download_task.download().await {
                for error in errors {
                    eprintln!("{error}");
                }
            }
            return;
        }
        "play" => {
            match args_vec.get(2) {
                Some(song) => {
                    if is_process {
                        let player: Player = match Player::single(song.to_string(), programdir) {
                            Ok(player) => player,
                            Err(e) => {
                                eprintln!("{e:?}");
                                return;
                            }
                        };
                        run_server(player);
                        return;
                    }
                }
                None => {
                    eprintln!("Expected play <song>");
                }
            }
        }
        _ => eprintln!("Unknown command: {command}")
    }
}
