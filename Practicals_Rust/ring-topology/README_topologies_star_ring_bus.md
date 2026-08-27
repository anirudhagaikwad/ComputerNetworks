# Star, Ring & Bus Topology Demos — Computer Networks Practical

Three more programs, all pure Rust `std` (no dependencies), each modeling a
**genuinely different real-world topology behavior** — not just the same
code relabeled:

| Topology | Who sees the full topology? | How does data travel? | Single point of failure |
|---|---|---|---|
| **Star** | Only the hub | Hub relays between spokes | The hub — kill it, every spoke loses its only link |
| **Ring** | Nobody — each node only knows its 2 neighbors | Hop-by-hop around the loop until it returns to the sender | Any one node — breaks both of its links at once |
| **Bus**  | Everybody — the segment broadcasts full status to all stations | One hop, direct broadcast to every other station | The shared segment (models the cable) |

All three were compiled and run in the practical environment to confirm
this behavior (connections forming live, messages relaying correctly,
topology views updating on connect/disconnect).

---

## Windows / Cargo setup (avoids the manifest issues from before)

Instead of fiddling with `[[bin]]` paths in `Cargo.toml`, put each file in
Cargo's **auto-discovered binaries folder**, `src/bin/` — Cargo picks up
every `.rs` file in there as its own runnable binary automatically, no
`Cargo.toml` edits needed at all:

```
peer-peer-con/
├── Cargo.toml
└── src/
    ├── main.rs                  (your existing 2-peer chat or mesh)
    └── bin/
        ├── star.rs               (rename p2p_star_topology.rs to this)
        ├── ring.rs               (rename p2p_ring_topology.rs to this)
        └── bus.rs                (rename p2p_bus_topology.rs to this)
```

Then run any of them with `cargo run --bin <name> -- <args>`, e.g.:
```powershell
cargo run --bin star -- Hub hub 8000 Bob,Carol
```

(If you'd rather not touch your existing `src/main.rs` setup at all, you
can just `rustc star.rs -o star.exe` etc. directly, same as before — no
Cargo needed.)

---

## 1. Star Topology

One **hub** process, N **spoke** processes. Spokes only ever talk to the
hub; the hub relays messages between them.

**Terminal 1 — Hub** (list the names of spokes you expect):
```powershell
cargo run --bin star -- Hub hub 8000 Bob,Carol
```

**Terminal 2 — Spoke Bob:**
```powershell
cargo run --bin star -- Bob spoke 127.0.0.1:8000
```

**Terminal 3 — Spoke Carol:**
```powershell
cargo run --bin star -- Carol spoke 127.0.0.1:8000
```

**What to observe:**
- The **hub's** screen redraws live with a full table (`Bob: CONNECTED`,
  `Carol: CONNECTED`, ...).
- Each **spoke's** screen only ever shows its own single line: `Link to
  Hub: CONNECTED`. It has no idea the other spoke exists — exactly how a
  real switch/hub port isolates leaf devices.
- Type a message in Bob's terminal → it's relayed through the hub and
  appears in Carol's terminal.
- `Ctrl+C` the Hub → **both** spokes immediately go `DOWN` — demonstrates
  the classic star single-point-of-failure.

---

## 2. Ring Topology

Every node connects to exactly two neighbors — a **successor** (outgoing
link) and a **predecessor** (incoming link) — forming a closed loop. Same
roster string in every terminal; the **order of names in the roster is the
ring order**.

```powershell
# Terminal 1
cargo run --bin ring -- Alice Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002

# Terminal 2
cargo run --bin ring -- Bob   Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002

# Terminal 3
cargo run --bin ring -- Carol Alice:127.0.0.1:9000,Bob:127.0.0.1:9001,Carol:127.0.0.1:9002
```

**What to observe:**
- Each node's screen shows exactly two lines: `Predecessor (...)` and
  `Successor (...)` — never a full topology table, because in a real ring
  a node genuinely only has two physical links.
- Type a message in Alice's terminal. Watch it print `Relaying message
  from Alice (hop 1)` in Bob's terminal, then `Relaying ... (hop 2)` in
  Carol's, and finally `Message completed the ring after 3 hop(s)` back in
  Alice's own terminal — the message visibly hops through every
  intermediate node instead of taking a direct link, just like a
  token-ring network.
- `Ctrl+C` Bob → Alice's `Successor` link and Carol's `Predecessor` link
  both drop, breaking the loop at that one point — shows how a ring is
  vulnerable to any single node failure (unless you have a redundant
  counter-rotating ring, which real Token Ring/FDDI networks sometimes
  add).

---

## 3. Bus Topology

All stations tap into one shared **segment** process (standing in for the
shared cable). Unlike the star hub, the segment doesn't just relay
messages — it continuously **broadcasts the full connected-station list to
every station**, because a real bus is one shared broadcast domain that
every tap can observe.

```powershell
# Terminal 1 — the shared segment (list expected station names)
cargo run --bin bus -- segment 6000 Alice,Bob,Carol

# Terminal 2
cargo run --bin bus -- Alice station 127.0.0.1:6000

# Terminal 3
cargo run --bin bus -- Bob station 127.0.0.1:6000

# Terminal 4
cargo run --bin bus -- Carol station 127.0.0.1:6000
```

**What to observe:**
- Every station's screen shows the **same full table** — unlike Star,
  where only the hub had that visibility.
- Type a message in any station's terminal — it reaches every other
  station in a single hop (no relaying through intermediate nodes, unlike
  Ring).
- `Ctrl+C` the segment → **every** station drops at once — models a break
  in the shared cable taking down the whole segment, the classic bus
  failure mode.

### Honesty note on the simulation
A real bus is a passive shared wire with no central device at all. A
genuine software equivalent would use OS-level UDP broadcast so every
station hears every packet directly with no relay — but that requires
multiple processes binding the *same* port with socket-reuse options,
which isn't available through Rust's plain `std` (would need the `socket2`
crate, breaking the "minimum dependency" goal). The single relay "segment"
process here is a deliberate, documented simplification that still
faithfully reproduces the two properties that matter for the practical:
everyone shares one broadcast domain, and one shared failure point takes
the whole segment down.

---

## Suggested write-up structure
For each topology: a short diagram (nodes + links), what you observed live
in the terminals, and the failure-mode test (which process did you kill,
and what happened). The three programs together let you demonstrate and
contrast all four topologies (Mesh, Star, Ring, Bus) from the same
practical using one consistent code style.
