// p2p_ring_topology.rs
// Ring topology demo: every node connects to exactly 2 neighbors —
// its successor (outgoing) and predecessor (incoming) — forming a closed loop.
// Pure std library — no dependencies.
//
// All nodes get the SAME roster string; the ORDER of entries defines the
// ring: Alice -> Bob -> Carol -> Alice (wraps around).
//
// Build: rustc p2p_ring_topology.rs -o ring
// Run (3 terminals, same roster string, only <my_name> differs):
//   ./ring Alice Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002
//   ./ring Bob   Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002
//   ./ring Carol Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002
//
// Data flows in ONE direction around the loop: a typed message goes to your
// successor, who forwards it to their successor, and so on, until it comes
// back to the original sender — like a token-ring network, where every
// intermediate node relays the frame instead of a direct link.

use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <my_name> <roster in ring order>", args[0]);
        eprintln!("roster = Name1:ip:port,Name2:ip:port,... (order = ring order)");
        std::process::exit(1);
    }
    let my_name = args[1].clone();
    let roster_str = args[2].clone();

    let roster: Vec<(String, String, String)> = roster_str
        .split(',')
        .map(|e| {
            let p: Vec<&str> = e.trim().splitn(3, ':').collect();
            if p.len() != 3 { panic!("Bad roster entry '{}'", e); }
            (p[0].to_string(), p[1].to_string(), p[2].to_string())
        })
        .collect();

    let n = roster.len();
    let my_index = roster.iter().position(|(nm, _, _)| nm == &my_name)
        .unwrap_or_else(|| panic!("'{}' not found in roster", my_name));
    let my_port = roster[my_index].2.clone();

    let (succ_name, succ_ip, succ_port) = roster[(my_index + 1) % n].clone();
    let (pred_name, _, _) = roster[(my_index + n - 1) % n].clone();

    let pred_status = Arc::new(Mutex::new(false));
    let succ_status = Arc::new(Mutex::new(false));
    let succ_writer: Arc<Mutex<Option<TcpStream>>> = Arc::new(Mutex::new(None));

    print_ring_topology(&my_name, &pred_name, false, &succ_name, false);

    // --- Listen for predecessor's incoming connection ---
    let listen_addr = format!("0.0.0.0:{}", my_port);
    let listener = TcpListener::bind(&listen_addr)
        .unwrap_or_else(|e| panic!("Failed to bind {}: {}", listen_addr, e));
    println!("[{}] Listening on {} (for predecessor: {})", my_name, listen_addr, pred_name);

    {
        let my_name = my_name.clone();
        let pred_name = pred_name.clone();
        let succ_name = succ_name.clone();
        let pred_status = Arc::clone(&pred_status);
        let succ_status = Arc::clone(&succ_status);
        let succ_writer = Arc::clone(&succ_writer);
        thread::spawn(move || {
            for incoming in listener.incoming() {
                if let Ok(stream) = incoming {
                    let mut reader = BufReader::new(stream);
                    *pred_status.lock().unwrap() = true;
                    print_ring_topology(&my_name, &pred_name, true, &succ_name, *succ_status.lock().unwrap());

                    let mut line = String::new();
                    loop {
                        line.clear();
                        match reader.read_line(&mut line) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => {
                                let trimmed = line.trim_end().to_string();
                                handle_ring_message(&my_name, &trimmed, &succ_writer);
                            }
                        }
                    }
                    *pred_status.lock().unwrap() = false;
                    print_ring_topology(&my_name, &pred_name, false, &succ_name, *succ_status.lock().unwrap());
                }
            }
        });
    }

    // --- Connect out to successor ---
    {
        let addr = format!("{}:{}", succ_ip, succ_port);
        let my_name = my_name.clone();
        let pred_name = pred_name.clone();
        let succ_name = succ_name.clone();
        let pred_status = Arc::clone(&pred_status);
        let succ_status = Arc::clone(&succ_status);
        let succ_writer = Arc::clone(&succ_writer);
        thread::spawn(move || loop {
            match TcpStream::connect(&addr) {
                Ok(stream) => {
                    *succ_writer.lock().unwrap() = Some(stream.try_clone().unwrap());
                    *succ_status.lock().unwrap() = true;
                    print_ring_topology(&my_name, &pred_name, *pred_status.lock().unwrap(), &succ_name, true);

                    let mut probe = BufReader::new(stream);
                    let mut junk = String::new();
                    loop {
                        junk.clear();
                        match probe.read_line(&mut junk) {
                            Ok(0) | Err(_) => break,
                            Ok(_) => {}
                        }
                    }
                    *succ_writer.lock().unwrap() = None;
                    *succ_status.lock().unwrap() = false;
                    print_ring_topology(&my_name, &pred_name, *pred_status.lock().unwrap(), &succ_name, false);
                    thread::sleep(Duration::from_secs(2));
                }
                Err(_) => thread::sleep(Duration::from_secs(2)),
            }
        });
    }

    println!("[{}] Type a message + Enter to send it around the ring. Type 'exit' to quit.\n", my_name);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = match line { Ok(m) => m, Err(_) => break };
        if msg.trim() == "exit" { break; }
        let frame = format!("MSG|{}|1|{}\n", my_name, msg);
        if let Some(w) = succ_writer.lock().unwrap().as_mut() {
            let _ = w.write_all(frame.as_bytes());
        } else {
            println!("(Successor link is down — message could not be sent)");
        }
    }
    println!("[{}] Exiting.", my_name);
}

fn handle_ring_message(my_name: &str, line: &str, succ_writer: &Arc<Mutex<Option<TcpStream>>>) {
    let parts: Vec<&str> = line.splitn(4, '|').collect();
    if parts.len() != 4 || parts[0] != "MSG" { return; }
    let origin = parts[1];
    let hop: u32 = parts[2].parse().unwrap_or(0);
    let text = parts[3];

    if origin == my_name {
        println!("[{}] Message completed the ring after {} hop(s): \"{}\"", my_name, hop, text);
        return;
    }

    println!("[{}] Relaying message from {} (hop {}): \"{}\"", my_name, origin, hop, text);
    let forward = format!("MSG|{}|{}|{}\n", origin, hop + 1, text);
    if let Some(w) = succ_writer.lock().unwrap().as_mut() {
        let _ = w.write_all(forward.as_bytes());
    }
}

fn print_ring_topology(my_name: &str, pred_name: &str, pred_up: bool, succ_name: &str, succ_up: bool) {
    print!("\x1B[2J\x1B[H");
    println!("=== RING TOPOLOGY — Node: {} ===", my_name);
    println!("Predecessor ({:<10}): {}", pred_name, if pred_up { "CONNECTED (incoming)" } else { "disconnected" });
    println!("Successor   ({:<10}): {}", succ_name, if succ_up { "CONNECTED (outgoing)" } else { "disconnected" });
    println!("--------------------------------------\n");
}
