use rodio::{OutputStream, Sink, Decoder};
use std::io::BufReader;
use std::fs::File;
use std::sync::{Arc, Mutex};

struct Player {
    songs: Vec<String>,
    queue: Sink,
    index: usize,
    commands: Arc<Mutex<Vec<String>>>
}

pub fn play_song(name: &String, programdir: &str) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = File::open(format!("{programdir}/{name}.mp3")).unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
