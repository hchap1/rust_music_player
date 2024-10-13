use crate::yt_search::{get_top_result, web_scrape};
use crate::yt_dlp::download_audio;
use crate::music::is_song_downloaded;
use std::collections::HashSet;
use std::fs::read_to_string;
use futures::future::join_all;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

pub async fn download_query(programdir: &str, query: &String, error_dump: Arc<Mutex<Vec<String>>>) {
    let html = match web_scrape(&format!("{query} song")).await {
        Ok(html) => html,
        Err(_) => return
    };
    let url = match get_top_result(&html) {
        Some(result) => result,
        None => panic!("No results found for query {query}")
    };
    
    if is_song_downloaded(programdir, query) {
        return;
    }
    
    match download_audio(&url, programdir, query.to_string()).await {
        Ok(_) => println!("Successfully downloaded {url}"),
        Err(e) => {
            let mut error_dump = error_dump.lock().unwrap();
            eprintln!("{e}");
            error_dump.push(e);
        }
    }
}

pub struct DownloadTask {
    queries: Vec<String>,
    directory: String,
}

impl DownloadTask {
    pub fn from_file(filepath: &String, programdir: String) -> Result<Self, String> {
        match read_to_string(filepath) {
            Ok(raw) => Ok(Self { 
                queries: raw.lines().map(|x| x.to_string()).collect::<Vec<String>>(), 
                directory: programdir 
            }),
            Err(e) => Err(format!("Failed to read {filepath}: {e:?}")),
        }
    }

    pub fn from_args(args_vec: Vec<String>, flags: HashSet<String>, programdir: String) -> Result<Self, String> {
        if flags.contains(&String::from("-f")) {
            let filepath: String = match args_vec.get(2) {
                Some(filepath) => filepath.to_string(),
                None => return Err(String::from("Expected download <FILEPATH> -f")),
            };
            Self::from_file(&filepath, programdir)
        } else {
            let song_names: Vec<String> = args_vec[2..].to_vec();
            Ok(Self { queries: song_names, directory: programdir })
        }
    }

    pub async fn download(&self) -> Result<String, Vec<String>> {
        let errors: Vec<String> = vec![];
        let errors = Arc::new(Mutex::new(errors));
        let semaphore = Arc::new(Semaphore::new(5));

        let futures: Vec<_> = self.queries.iter().map(|x| {
            let permit = Arc::clone(&semaphore).acquire_owned();
            let errors = Arc::clone(&errors);
            let dir = self.directory.clone();
            async move {
                let _permit = permit.await.unwrap();
                download_query(&dir, x, errors).await
            }
        }).collect();

        join_all(futures).await;

        let len = {
            let errors = errors.lock().unwrap();
            errors.len()
        };
        
        match len {
            0 => Ok(format!("Successfully downloaded {} songs", self.queries.len())),
            _ => Err(errors.lock().unwrap().to_vec()),
        }
    }
}
