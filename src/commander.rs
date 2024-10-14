use std::net::TcpStream;
use std::io::Write;
use std::process::Command;
use std::env::current_exe;

pub struct CommanderController {
    stream: TcpStream
}

pub fn launch_process(mut args: Vec<String>) {
    let current_exe: String = match current_exe() {
        Ok(pathbuf) => {
            pathbuf.to_string_lossy().to_string()
        }
        Err(_) => {
            eprintln!("Could not retrieve EXE location.");
            return;
        }
    };
    println!("About to start command: {current_exe}");
    args.remove(0);
    args.push(String::from("-process"));
    let _ = Command::new(current_exe)
        .args(&args[..])
        .spawn();
}

impl CommanderController {
    pub fn connect() -> Option<CommanderController> {
        match TcpStream::connect("127.0.0.1:7878") {
            Ok(stream) => {
                Some(Self { stream })
            }
            Err(_) => {
                None
            }
        }
    }
    
    pub fn send_command(&mut self, command: &String) {
        let _ = self.stream.write_all(command.as_bytes());
    }
}
