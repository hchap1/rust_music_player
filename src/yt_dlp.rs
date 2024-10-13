use std::process;
use tokio::process::Command;

 pub fn open_folder(path: &String) {
    let _ = process::Command::new("explorer")
        .arg(path)
        .output()
        .expect("Failed to execute command");
}

pub async fn download_audio(url: &String, path: &str) -> Result<String, String> {
    println!("DOWNLOADING {url}");
    
    let output = Command::new("yt-dlp")
        .args(&["-x", "-P", path, "--audio-format", "mp3", url])
        .output()
        .await
        .map_err(|e| format!("Failed to execute command: {e:?}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
