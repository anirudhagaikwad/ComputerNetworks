# P2P Chat — Computer Networks Practical (Rust)

A real-time peer-to-peer (P2P) communication demo built with **only the Rust
standard library** — zero external crates, zero `Cargo.toml` needed.

## Why this counts as "P2P" (not client-server)
Each running instance is simultaneously:
- a **server** — a `TcpListener` accepting incoming connections, and
- a **client** — a `TcpStream` that connects out to the other peer.

There is no central/coordinating server. Both nodes are equal peers, which is
the defining property of a P2P architecture (as opposed to a client-server
model like a normal web app).

## Requirements
- Rust / `rustc` installed (works with 1.96.0 or any recent stable version —
  the program only uses long-stable std APIs: `TcpListener`, `TcpStream`,
  `thread`, `io`).
- No internet access or crates.io needed at build time.

## Minimum footprint
- 1 source file, ~70 lines.
- No `Cargo.toml`, no dependency downloads — compiled directly with `rustc`.
- Binary only depends on the OS's native TCP/IP stack (via libc/std).

## Steps to run

### 1. Compile & Run Run two peers (same machine, two terminals)

**Terminal 1 (Peer "Anirudha"):**
```bash
cargo run Anirudha 7878 127.0.0.1:7879
```

**Terminal 2 (Peer "Bubbu"):**
```bash
cargo run Bubbu 7879 127.0.0.1:7878
```

Arguments: `<your_name> <your_listen_port> <peer_ip:peer_port>`

Each peer will print "Waiting for peer..." until the other one starts —
start both within a few seconds of each other (or just re-run whichever
starts first; it retries automatically).

### 3. Chat
Type a message in either terminal and hit Enter — it appears instantly in
the other terminal. Type `exit` to close a peer.

### 4. Running across two real machines (LAN)
1. Find each machine's local IP (`ip addr` on Linux, `ipconfig` on Windows).
2. On Machine A: `./main Anirudha 7878 <Machine_B_IP>:7879`
3. On Machine B: `./main Bubbu 7879 <Machine_A_IP>:7878`
4. Make sure the listen port is allowed through each machine's firewall.

## What to point out in your practical write-up
- `TcpListener::bind` — opens a socket in LISTEN state (server side of the
  3-way handshake).
- `TcpStream::connect` — initiates the TCP 3-way handshake as a client.
- Two independent TCP connections are used (A→B and B→A) so both directions
  work concurrently without blocking each other — demonstrated with
  `std::thread`.
- Since it's plain TCP, you can observe the handshake/packets with
  `netstat -an | grep 7878` or Wireshark filtering on the chosen port.
