use regex::Regex;
use std::fs::read_dir;

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
