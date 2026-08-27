// p2p_bus_topology.rs
// Bus topology demo: all stations tap into one shared "bus segment"
// (modeled as a single relay process — the closest safe-Rust, single-machine
// equivalent of a shared coax cable; genuine OS-level broadcast to multiple
// processes on one port needs socket options beyond plain std). Pure std
// library — no dependencies.
//
// KEY DIFFERENCE FROM STAR: every station sees the FULL list of who's on
// the segment (the segment continuously broadcasts topology state to
// everyone), because a real bus is one shared broadcast domain — every tap
// can observe who else is active. In the star demo, only the hub had that
// visibility; spokes only saw their own link.
//
// KEY DIFFERENCE FROM RING: a message is broadcast directly and immediately
// to every OTHER station in one hop — no relaying through intermediate
// nodes.
//
// Segment: rustc p2p_bus_topology.rs -o bus
//          ./bus segment 6000 Alice,Bob,Carol      (expected station names)
// Station: ./bus Alice station 127.0.0.1:6000
//          ./bus Bob   station 127.0.0.1:6000

use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  Segment: {} segment <port> <expected_station_names_comma_separated>", args[0]);
        eprintln!("  Station: {} <name> station <segment_ip:segment_port>", args[0]);
        std::process::exit(1);
    }

    if args[1] == "segment" {
        run_segment(&args);
    } else {
        run_station(&args);
    }
}

// ---------------- BUS SEGMENT (shared medium relay) ----------------

fn run_segment(args: &[String]) {
    if args.len() != 4 {
        eprintln!("Usage: {} segment <port> <expected_station_names_comma_separated>", args[0]);
        std::process::exit(1);
    }
    let port = &args[2];
    let expected: Vec<String> = args[3].split(',').map(|s| s.trim().to_string()).collect();

    let status: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(
        expected.iter().map(|n| (n.clone(), false)).collect(),
    ));
    let streams: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&listen_addr)
        .unwrap_or_else(|e| panic!("Failed to bind {}: {}", listen_addr, e));
    println!("[BUS SEGMENT] Listening on {}", listen_addr);
    broadcast_topology(&status, &streams);

    for incoming in listener.incoming() {
        let status = Arc::clone(&status);
        let streams = Arc::clone(&streams);
        if let Ok(stream) = incoming {
            thread::spawn(move || {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 { return; }
                let station_name = line.trim().to_string();

                streams.lock().unwrap().insert(station_name.clone(), stream);
                status.lock().unwrap().insert(station_name.clone(), true);
                broadcast_topology(&status, &streams);

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
                                if name != &station_name && s.write_all(msg.as_bytes()).is_err() {
                                    dead.push(name.clone());
                                }
                            }
                            for d in dead { map.remove(&d); }
                        }
                    }
                }

                status.lock().unwrap().insert(station_name.clone(), false);
                streams.lock().unwrap().remove(&station_name);
                broadcast_topology(&status, &streams);
            });
        }
    }
}

fn broadcast_topology(status: &Arc<Mutex<HashMap<String, bool>>>, streams: &Arc<Mutex<HashMap<String, TcpStream>>>) {
    let map = status.lock().unwrap();
    let mut names: Vec<&String> = map.keys().collect();
    names.sort();
    let body: Vec<String> = names.iter().map(|n| format!("{}:{}", n, map.get(*n).unwrap())).collect();
    let line = format!("TOPO {}\n", body.join(","));
    drop(map);

    let mut smap = streams.lock().unwrap();
    let mut dead = vec![];
    for (name, s) in smap.iter_mut() {
        if s.write_all(line.as_bytes()).is_err() {
            dead.push(name.clone());
        }
    }
    for d in dead { smap.remove(&d); }
}

// ---------------- STATION ----------------

fn run_station(args: &[String]) {
    if args.len() != 4 || args[2] != "station" {
        eprintln!("Usage: {} <name> station <segment_ip:segment_port>", args[0]);
        std::process::exit(1);
    }
    let my_name = args[1].clone();
    let segment_addr = args[3].clone();

    loop {
        match TcpStream::connect(&segment_addr) {
            Ok(mut stream) => {
                let announce = format!("{}\n", my_name);
                if stream.write_all(announce.as_bytes()).is_err() {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
                let reader = BufReader::new(stream.try_clone().unwrap());
                let my_name_clone = my_name.clone();
                thread::spawn(move || {
                    let mut reader = reader;
                    let mut line = String::new();
                    loop {
                        line.clear();
                        match reader.read_line(&mut line) {
                            Ok(0) | Err(_) => {
                                println!("\n[{}] Link to bus segment: DOWN", my_name_clone);
                                break;
                            }
                            Ok(_) => {
                                let trimmed = line.trim_end();
                                if let Some(rest) = trimmed.strip_prefix("TOPO ") {
                                    print_bus_topology(&my_name_clone, rest);
                                } else if let Some(text) = trimmed.strip_prefix("MSG ") {
                                    println!("{}", text);
                                }
                            }
                        }
                    }
                });

                println!("[{}] Type a message + Enter to broadcast on the bus. Type 'exit' to quit.\n", my_name);
                let stdin = io::stdin();
                let mut broken = false;
                for line in stdin.lock().lines() {
                    let msg = match line { Ok(m) => m, Err(_) => break };
                    if msg.trim() == "exit" { return; }
                    let full = format!("MSG {}: {}\n", my_name, msg);
                    if stream.write_all(full.as_bytes()).is_err() {
                        broken = true;
                        break;
                    }
                }
                if broken {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
                return;
            }
            Err(_) => {
                println!("[{}] Bus segment not reachable — retrying...", my_name);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn print_bus_topology(my_name: &str, body: &str) {
    print!("\x1B[2J\x1B[H");
    println!("=== BUS TOPOLOGY — Station: {} ===", my_name);
    println!("(Every station shares this SAME view — the whole segment is one broadcast domain)\n");
    println!("{:<15} {}", "Station", "Status");
    println!("{:<15} {}", "-------", "------");
    let mut active = 0;
    for entry in body.split(',') {
        let parts: Vec<&str> = entry.splitn(2, ':').collect();
        if parts.len() != 2 { continue; }
        let up = parts[1] == "true";
        if up { active += 1; }
        println!("{:<15} {}", parts[0], if up { "CONNECTED" } else { "disconnected" });
    }
    println!("-----------------------------------");
    println!("Active on bus: {}\n", active);
}
