use rodio::{OutputStream, Sink, Decoder};
use std::io::BufReader;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::fs::read_dir;
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;

pub fn is_song_downloaded(path: &str, song_name: &String) -> bool {
    let regex = Regex::new(song_name).unwrap();
    match read_dir(path) {
        Ok(contents) => {
            for item in contents {
                if let Err(_) = item {
                    continue;
                }
                let item = item.unwrap();
                let filename = item.file_name().to_string_lossy().to_string();
                if regex.is_match(&filename) {
                    return true;
                }
            }
            false
        }
        Err(_) => false
    }
}

pub fn load_songs(programdir: &str) -> Result<Vec<String>, String> {
    let mut songs_list: Vec<String> = vec![];
    match read_dir(programdir) {
        Ok(contents) => {
            for item in contents {
                if let Ok(file) = item {
                    songs_list.push(file.file_name().to_string_lossy().to_string());
                }
            }
        }
        Err(e) => {
            return Err(format!("Could not read directory: {e:?}"));
        }
    }
    Ok(songs_list)
}

pub fn shuffle(songs: &mut Vec<String>) {
    let mut rng = thread_rng();
    songs.shuffle(&mut rng);
}

pub struct Player {
    songs: Vec<String>,
    queue: Sink,
    index: usize,
    commands: Arc<Mutex<Vec<String>>>
}

impl Player {
    fn single(song: String, programdir: String) -> Result<Self, String> {
        match is_song_downloaded(&programdir.as_str(), &song) {
            true => {
                let songs = vec![song];
                let sink: Sink = match play_songs(&songs, programdir.as_str()) {
                    Ok(sink) => sink,
                    Err(_) => {
                        return Err(String::from("Failed to play song."));
                    }
                };
                Ok(Self {
                    songs,
                    queue: sink,
                    index: 0,
                    commands: Arc::new(Mutex::new(vec![]))
                })
            }
            false => {
                return Err(String::from("Song is not downloaded."));
            }
        }
    }
}

pub fn play_songs(names: &Vec<String>, programdir: &str) -> Result<Sink, String> {
    let (_stream, handle) = match OutputStream::try_default() {
        Ok(output_stream) => output_stream,
        Err(e) => return Err(format!("{e:?}"))
    };
    let sink = match Sink::try_new(&handle) {
        Ok(sink) => sink,
        Err(e) => return Err(format!("{e:?}"))
    };
    for name in names {
        let file = match File::open(format!("{programdir}/{name}.mp3")) {
            Ok(file) => file,
            Err(_) => { eprintln!("No such song: {name}.mp3"); continue; }
        };
        sink.append(Decoder::new(BufReader::new(file)).unwrap());
    }
    Ok(sink)
}
