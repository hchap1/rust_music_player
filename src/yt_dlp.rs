use std::process::Command;

 pub fn open_folder(path: &String) {
    let _ = Command::new("explorer")
        .arg(path)
        .output()
        .expect("Failed to execute command");
}

pub fn download_audio(url: &String, path: &String) -> Result<String, String> {
    println!("DOWNLOADING {url}");
    let output = Command::new("yt-dlp")
        .args(&["-x", "-P", path, "--audio-format", "mp3", url])
        .output()
        .expect("Failed to execute command");
    match output.status.success() {
        true => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        false => Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
