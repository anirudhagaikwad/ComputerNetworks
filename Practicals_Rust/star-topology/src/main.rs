// p2p_star_topology.rs
// Star topology demo: one Hub + many Spokes, connected only Hub<->Spoke.
// Pure std library — no dependencies.
//
// Hub:   rustc p2p_star_topology.rs -o star
//        ./star Hub hub 8000 Bob,Carol,Dave      (expected spoke names)
// Spoke: ./star Bob spoke 127.0.0.1:8000
//        ./star Carol spoke 127.0.0.1:8000
//
// Key property demonstrated: only the HUB sees the full topology.
// Each SPOKE only ever knows the status of its OWN single link to the hub —
// exactly like a real star network, where leaf devices can't see each other
// directly, only through the central device.

use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  Hub:   {} <name> hub <port> <expected_spoke_names_comma_separated>", args[0]);
        eprintln!("  Spoke: {} <name> spoke <hub_ip:hub_port>", args[0]);
        std::process::exit(1);
    }
    let my_name = args[1].clone();
    let role = args[2].clone();

    match role.as_str() {
        "hub" => run_hub(&my_name, &args),
        "spoke" => run_spoke(&my_name, &args),
        _ => {
            eprintln!("Unknown role '{}': use 'hub' or 'spoke'", role);
            std::process::exit(1);
        }
    }
}

// ---------------- HUB ----------------

fn run_hub(my_name: &str, args: &[String]) {
    if args.len() != 5 {
        eprintln!("Usage: {} <name> hub <port> <expected_spoke_names_comma_separated>", args[0]);
        std::process::exit(1);
    }
    let port = &args[3];
    let expected: Vec<String> = args[4].split(',').map(|s| s.trim().to_string()).collect();

    let status: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(
        expected.iter().map(|n| (n.clone(), false)).collect(),
    ));
    let streams: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    print_hub_topology(my_name, &status);

    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&listen_addr)
        .unwrap_or_else(|e| panic!("Failed to bind {}: {}", listen_addr, e));
    println!("[HUB {}] Listening on {}", my_name, listen_addr);

    for incoming in listener.incoming() {
        let status = Arc::clone(&status);
        let streams = Arc::clone(&streams);
        let my_name = my_name.to_string();
        if let Ok(stream) = incoming {
            thread::spawn(move || {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 {
                    return;
                }
                let spoke_name = line.trim().to_string();

                streams.lock().unwrap().insert(spoke_name.clone(), stream);
                status.lock().unwrap().insert(spoke_name.clone(), true);
                print_hub_topology(&my_name, &status);

                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => {
                            let msg = line.clone();
                            let mut map = streams.lock().unwrap();
                            let mut dead = vec![];
                            for (name, s) in map.iter_mut() {
                                if name != &spoke_name && s.write_all(msg.as_bytes()).is_err() {
                                    dead.push(name.clone());
                                }
                            }
                            for d in dead {
                                map.remove(&d);
                            }
                        }
                    }
                }

                status.lock().unwrap().insert(spoke_name.clone(), false);
                streams.lock().unwrap().remove(&spoke_name);
                print_hub_topology(&my_name, &status);
            });
        }
    }
}

fn print_hub_topology(hub_name: &str, status: &Arc<Mutex<HashMap<String, bool>>>) {
    let map = status.lock().unwrap();
    let mut names: Vec<&String> = map.keys().collect();
    names.sort();
    print!("\x1B[2J\x1B[H");
    println!("=== STAR TOPOLOGY — Hub: {} ===", hub_name);
    println!("{:<15} {}", "Spoke", "Status");
    println!("{:<15} {}", "-----", "------");
    let mut active = 0;
    for name in &names {
        let c = *map.get(*name).unwrap();
        if c { active += 1; }
        println!("{:<15} {}", name, if c { "CONNECTED" } else { "disconnected" });
    }
    println!("-----------------------------------");
    println!("Active spokes: {} / {}", active, names.len());
    println!("(Spokes cannot see each other — only the hub has this full view)\n");
}

// ---------------- SPOKE ----------------

fn run_spoke(my_name: &str, args: &[String]) {
    if args.len() != 4 {
        eprintln!("Usage: {} <name> spoke <hub_ip:hub_port>", args[0]);
        std::process::exit(1);
    }
    let hub_addr = args[3].clone();

    loop {
        println!("=== STAR TOPOLOGY — Spoke: {} ===", my_name);
        println!("Connecting to hub at {}...", hub_addr);
        match TcpStream::connect(&hub_addr) {
            Ok(mut stream) => {
                let announce = format!("{}\n", my_name);
                if stream.write_all(announce.as_bytes()).is_err() {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
                println!("Link to Hub: CONNECTED\n");

                let reader = BufReader::new(stream.try_clone().unwrap());
                let my_name_clone = my_name.to_string();
                thread::spawn(move || {
                    let mut reader = reader;
                    let mut line = String::new();
                    loop {
                        line.clear();
                        match reader.read_line(&mut line) {
                            Ok(0) | Err(_) => {
                                println!("\n[{}] Link to Hub: DOWN", my_name_clone);
                                break;
                            }
                            Ok(_) => {
                                let trimmed = line.trim_end();
                                if let Some(text) = trimmed.strip_prefix("MSG ") {
                                    println!("{}", text);
                                }
                            }
                        }
                    }
                });

                println!("Type a message + Enter (goes to Hub, relayed to other spokes). Type 'exit' to quit.\n");
                let stdin = io::stdin();
                let mut broken = false;
                for line in stdin.lock().lines() {
                    let msg = match line { Ok(m) => m, Err(_) => break };
                    if msg.trim() == "exit" {
                        return;
                    }
                    let full = format!("MSG {}: {}\n", my_name, msg);
                    if stream.write_all(full.as_bytes()).is_err() {
                        broken = true;
                        break;
                    }
                }
                if broken {
                    println!("Link to Hub: DOWN — retrying...");
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
                return;
            }
            Err(_) => {
                println!("Link to Hub: DOWN (hub not reachable) — retrying...\n");
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}
