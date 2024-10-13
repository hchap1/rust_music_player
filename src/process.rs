use crate::music::Player;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn receive_commands(mut stream: TcpStream, commander: Arc<Mutex<Commander>>, message_stack: Arc<Mutex<Vec<String>>>) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let mut commander = commander.lock().unwrap();
                if bytes_read > 0 {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let mut message_stack = message_stack.lock().unwrap();
                    message_stack.push(message.to_string());
                    commander.new_com = true;
                } else {
                    commander.alive = false;
                    return;
                }
            }
            Err(_) => {
                let mut commander = commander.lock().unwrap();
                commander.alive = false;
                return;
            }
        }
    }
}

struct Commander {
    new_com: bool,
    alive: bool
}

pub fn run_server(programdir: &str) {
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => {
            let mut message_dump: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
            let mut dump_ref = Arc::clone(&message_dump);
            loop {
                let (recv_stream, addr) = match listener.accept() {
                    Ok((s, a)) => (s, a),
                    Err(_) => {
                        eprintln!("Could not accept commander.");
                        return;
                    }
                };
                let send_stream = recv_stream.try_clone().unwrap();
                let commander = Arc::new(Mutex::new(Commander{ new_com: false, alive: true }));
                let commander_ref = Arc::clone(&commander);
                let commander_thread = spawn(|| {
                    receive_commands(recv_stream, commander_ref, dump_ref);
                });
                loop {
                    sleep(Duration::from_millis(10));
                    {
                        let commander = commander.lock().unwrap();
                        if commander.new_com {
                            
                        }
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("Could not start server.");
            return;
        }
    }
}
