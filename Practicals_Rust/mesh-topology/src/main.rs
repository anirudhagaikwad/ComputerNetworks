// p2p_mesh_topology.rs
// Configurable N-peer full-mesh P2P network with a live console topology view.
// Pure std library — no external dependencies.
//
// Every peer is given the SAME roster string (all peers in the network,
// including itself). Each node figures out its own port from the roster,
// and connects to every peer whose name is lexicographically GREATER than
// its own (the peer with the smaller name always initiates the connection
// — this avoids duplicate connections in a full mesh, so N peers form
// exactly N*(N-1)/2 edges, not N*(N-1)).
//
// Build:  rustc p2p_mesh_topology.rs -o p2p_mesh
// Run:    ./p2p_mesh <my_name> <roster>
//   roster = "Name1:ip:port,Name2:ip:port,Name3:ip:port,..."
//
// Example (3 terminals, SAME roster string in all three, only <my_name> differs):
//   ./p2p_mesh Anirudha Anirudha:127.0.0.1:7878,Bubbu:127.0.0.1:7879,Cairi:127.0.0.1:7880
//   ./p2p_mesh Bubbu   Anirudha:127.0.0.1:7878,Bubbu:127.0.0.1:7879,Cairi:127.0.0.1:7880
//   ./p2p_mesh Cairi Anirudha:127.0.0.1:7878,Bubbu:127.0.0.1:7879,Cairi:127.0.0.1:7880

use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Status = Arc<Mutex<HashMap<String, bool>>>;
type Streams = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <my_name> <roster>", args[0]);
        eprintln!("roster = Name1:ip:port,Name2:ip:port,...");
        std::process::exit(1);
    }
    let my_name = args[1].clone();
    let roster_str = args[2].clone();

    let roster: Vec<(String, String, String)> = roster_str
        .split(',')
        .map(|entry| {
            let parts: Vec<&str> = entry.trim().splitn(3, ':').collect();
            if parts.len() != 3 {
                panic!("Bad roster entry '{}': expected Name:ip:port", entry);
            }
            (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            )
        })
        .collect();

    let my_entry = roster
        .iter()
        .find(|(n, _, _)| n == &my_name)
        .unwrap_or_else(|| panic!("'{}' not found in roster", my_name));
    let my_port = my_entry.2.clone();

    let others: Vec<(String, String, String)> = roster
        .iter()
        .filter(|(n, _, _)| n != &my_name)
        .cloned()
        .collect();

    let status: Status = Arc::new(Mutex::new(
        others.iter().map(|(n, _, _)| (n.clone(), false)).collect(),
    ));
    let streams: Streams = Arc::new(Mutex::new(HashMap::new()));

    print_topology(&my_name, &status);

    // --- Listener: accept incoming edges from peers with a "smaller" name ---
    let listen_addr = format!("0.0.0.0:{}", my_port);
    let listener = TcpListener::bind(&listen_addr)
        .unwrap_or_else(|e| panic!("Failed to bind {}: {}", listen_addr, e));
    println!("[{}] Listening on {}", my_name, listen_addr);

    {
        let status = Arc::clone(&status);
        let streams = Arc::clone(&streams);
        let my_name_clone = my_name.clone();
        thread::spawn(move || {
            for incoming in listener.incoming() {
                let status = Arc::clone(&status);
                let streams = Arc::clone(&streams);
                let my_name = my_name_clone.clone();
                if let Ok(stream) = incoming {
                    thread::spawn(move || {
                        let mut reader = BufReader::new(stream.try_clone().unwrap());
                        let mut line = String::new();
                        if reader.read_line(&mut line).unwrap_or(0) == 0 {
                            return;
                        }
                        let peer_name = line.trim().to_string();
                        register_connection(&my_name, &peer_name, stream, &status, &streams);
                        read_loop(&my_name, &peer_name, reader, &status, &streams);
                    });
                }
            }
        });
    }

    // --- Outgoing: connect to every peer whose name is lexicographically greater ---
    for (peer_name, ip, port) in others
        .iter()
        .filter(|(n, _, _)| n.as_str() > my_name.as_str())
    {
        let addr = format!("{}:{}", ip, port);
        let status = Arc::clone(&status);
        let streams = Arc::clone(&streams);
        let my_name_clone = my_name.clone();
        let peer_name = peer_name.clone();
        thread::spawn(move || loop {
            match TcpStream::connect(&addr) {
                Ok(mut stream) => {
                    let announce = format!("{}\n", my_name_clone);
                    if stream.write_all(announce.as_bytes()).is_err() {
                        thread::sleep(Duration::from_secs(2));
                        continue;
                    }
                    let reader = BufReader::new(stream.try_clone().unwrap());
                    register_connection(&my_name_clone, &peer_name, stream, &status, &streams);
                    read_loop(&my_name_clone, &peer_name, reader, &status, &streams);
                    thread::sleep(Duration::from_secs(2)); // dropped — retry
                }
                Err(_) => thread::sleep(Duration::from_secs(2)),
            }
        });
    }

    println!(
        "[{}] Type a message + Enter to broadcast to all connected peers. Type 'exit' to quit.\n",
        my_name
    );

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = match line {
            Ok(m) => m,
            Err(_) => break,
        };
        if msg.trim() == "exit" {
            break;
        }
        let full = format!("MSG {}: {}\n", my_name, msg);
        let mut map = streams.lock().unwrap();
        let mut dead = vec![];
        for (peer, s) in map.iter_mut() {
            if s.write_all(full.as_bytes()).is_err() {
                dead.push(peer.clone());
            }
        }
        for d in dead {
            map.remove(&d);
        }
    }
    println!("[{}] Exiting.", my_name);
}

fn register_connection(
    my_name: &str,
    peer_name: &str,
    stream: TcpStream,
    status: &Status,
    streams: &Streams,
) {
    streams
        .lock()
        .unwrap()
        .insert(peer_name.to_string(), stream);
    status.lock().unwrap().insert(peer_name.to_string(), true);
    print_topology(my_name, status);
}

fn read_loop(
    my_name: &str,
    peer_name: &str,
    mut reader: BufReader<TcpStream>,
    status: &Status,
    streams: &Streams,
) {
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break, // peer disconnected
            Ok(_) => {
                let trimmed = line.trim_end();
                if let Some(text) = trimmed.strip_prefix("MSG ") {
                    println!("{}", text);
                }
            }
        }
    }
    status.lock().unwrap().insert(peer_name.to_string(), false);
    streams.lock().unwrap().remove(peer_name);
    print_topology(my_name, status);
}

fn print_topology(my_name: &str, status: &Status) {
    let map = status.lock().unwrap();
    let mut names: Vec<&String> = map.keys().collect();
    names.sort();

    // Clears the terminal and redraws — gives a "live updating" effect.
    print!("\x1B[2J\x1B[H");
    println!("=== P2P Mesh Topology — Node: {} ===", my_name);
    println!("{:<15} {}", "Peer", "Status");
    println!("{:<15} {}", "----", "------");
    let mut active = 0;
    for name in &names {
        let connected = *map.get(*name).unwrap();
        if connected {
            active += 1;
        }
        println!(
            "{:<15} {}",
            name,
            if connected {
                "CONNECTED"
            } else {
                "disconnected"
            }
        );
    }
    println!("--------------------------------------");
    println!("Active edges from this node: {} / {}", active, names.len());
    println!();
}
