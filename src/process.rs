use crate::music::Player;
use std::fs::File;
use rodio::{OutputStream, Sink, Decoder};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};

struct Commander {
    new_com: bool,
    alive: bool,
}

fn receive_commands(
    mut stream: TcpStream,
    commander: Arc<Mutex<Commander>>,
    message_stack: Arc<Mutex<Vec<String>>>,
) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let mut message_stack = message_stack.lock().unwrap();
                    message_stack.push(message.to_string());
                    let mut commander = commander.lock().unwrap();
                    commander.new_com = true;
                } else {
                    let mut commander = commander.lock().unwrap();
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

fn listen_for_clients(
    listener: TcpListener,
    message_stack: Arc<Mutex<Vec<String>>>,
) {
    let mut commanders = Vec::new();

    loop {
        match listener.accept() {
            Ok((recv_stream, _addr)) => {
                let new_commander = Arc::new(Mutex::new(Commander {
                    new_com: false,
                    alive: true,
                }));

                let commander_clone = Arc::clone(&new_commander);
                let message_stack_clone = Arc::clone(&message_stack);
                let recv_stream_clone = recv_stream.try_clone().unwrap();

                spawn(move || {
                    receive_commands(recv_stream_clone, commander_clone, message_stack_clone);
                });

                commanders.push(new_commander);
            }
            Err(e) => {
                println!("Error accepting connection: {e}");
            }
        }
    }
}

pub fn run_server(programdir: String, player: Player) {
    let (_stream, handle) = match OutputStream::try_default() {
        Ok(output_stream) => output_stream,
        Err(e) => {
            eprintln!("Error initializing output stream: {e}");
            return;
        }
    };
    let sink = match Sink::try_new(&handle) {
        Ok(sink) => sink,
        Err(e) => {
            eprintln!("Error creating sink: {e}");
            return;
        }
    };
    for name in &player.songs {
        let file = match File::open(format!("{programdir}/{name}.mp3")) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("No such song: {name}.mp3");
                continue;
            }
        };
        sink.append(Decoder::new(BufReader::new(file)).unwrap());
    }
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!("Could not start server.");
            return;
        }
    };
    let message_stack: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let listener_message_stack = Arc::clone(&message_stack);
    spawn(move || {
        listen_for_clients(listener, listener_message_stack);
    });

    loop {
        sleep(Duration::from_millis(100));
        
        let mut message_stack = message_stack.lock().unwrap();
        if message_stack.is_empty() {
            continue;
        }
        for message in message_stack.drain(..) {
            match message.as_str() {
                "pause" => {
                    sink.pause();
                }
                "resume" => {
                    sink.play();
                }
                "stop" => {
                    sink.stop();
                    return;
                }
                _ => {}
            }
        }
    }
}
