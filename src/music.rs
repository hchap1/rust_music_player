use rodio::{OutputStream, Sink, Decoder};
use std::io::BufReader;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::fs::read_dir;
use rand::{seq::SliceRandom, thread_rng};

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

}

pub fn play_song(name: &String, programdir: &str) {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = File::open(format!("{programdir}/{name}.mp3")).unwrap();
    sink.append(Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
