// p2p_chat.rs
// Minimal-footprint Peer-to-Peer (P2P) communication demo.
// Uses ONLY the Rust standard library (no external crates/dependencies).
//
// Concept: Each running instance is a "peer" that is BOTH a server (listens
// for incoming connections) and a client (connects out to another peer).
// There is no central server — this is what makes it peer-to-peer.
//
// Build:  rustc p2p_chat.rs -o p2p_chat
// Run:    ./p2p_chat <your_name> <your_listen_port> <peer_ip:peer_port>

use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <name> <listen_port> <peer_ip:peer_port>", args[0]);
        eprintln!("Example: {} Alice 7878 127.0.0.1:7879", args[0]);
        std::process::exit(1);
    }

    let name = args[1].clone();
    let listen_port = args[2].clone();
    let peer_addr = args[3].clone();

    // --- Server role: listen for incoming messages from the peer ---
    let listen_addr = format!("0.0.0.0:{}", listen_port);
    let listener = TcpListener::bind(&listen_addr)
        .unwrap_or_else(|e| panic!("Failed to bind on {}: {}", listen_addr, e));
    println!("[{}] Listening on {}", name, listen_addr);

    thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => {
                    thread::spawn(move || handle_incoming(stream));
                }
                Err(e) => eprintln!("Incoming connection error: {}", e),
            }
        }
    });

    // --- Client role: connect out to the peer (retry until peer is ready) ---
    let mut out_stream;
    loop {
        match TcpStream::connect(&peer_addr) {
            Ok(s) => {
                out_stream = s;
                break;
            }
            Err(_) => {
                println!("[{}] Waiting for peer at {}...", name, peer_addr);
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
    println!("[{}] Connected to peer at {}", name, peer_addr);
    println!("Type a message and press Enter to send. Type 'exit' to quit.\n");

    // --- Main thread: read from stdin, send over the socket ---
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = match line {
            Ok(m) => m,
            Err(_) => break,
        };
        if msg.trim() == "exit" {
            break;
        }
        let full_msg = format!("{}: {}\n", name, msg);
        if let Err(e) = out_stream.write_all(full_msg.as_bytes()) {
            eprintln!("Send failed (peer may have disconnected): {}", e);
            break;
        }
    }

    println!("[{}] Exiting.", name);
}

// Reads lines coming in from the peer and prints them to the console.
fn handle_incoming(stream: TcpStream) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(msg) => println!("{}", msg),
            Err(_) => break,
        }
    }
}
