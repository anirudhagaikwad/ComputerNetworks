# Ring Topology Demos — Computer Networks Practical


| Topology | Who sees the full topology? | How does data travel? | Single point of failure |
|---|---|---|---|
| **Ring** | Nobody — each node only knows its 2 neighbors | Hop-by-hop around the loop until it returns to the sender | Any one node — breaks both of its links at once |


compiled and run in the practical environment to confirm
this behavior (connections forming live, messages relaying correctly,
topology views updating on connect/disconnect).

---

## 2. Ring Topology

Every node connects to exactly two neighbors — a **successor** (outgoing
link) and a **predecessor** (incoming link) — forming a closed loop. Same
roster string in every terminal; the **order of names in the roster is the
ring order**.

```powershell
# Terminal 1
cargo run -- Anirudha Anirudha:127.0.0.1:9000,Bubbu:127.0.0.1:9001,Cairi:127.0.0.1:9002

# Terminal 2
cargo run -- Bubbu   Anirudha:127.0.0.1:9000,Bubbu:127.0.0.1:9001,Cairi:127.0.0.1:9002

# Terminal 3
cargo run -- Cairi Anirudha:127.0.0.1:9000,Bubbu:127.0.0.1:9001,Cairi:127.0.0.1:9002
```

**What to observe:**
- Each node's screen shows exactly two lines: `Predecessor (...)` and
  `Successor (...)` — never a full topology table, because in a real ring
  a node genuinely only has two physical links.
- Type a message in Anirudha's terminal. Watch it print `Relaying message
  from Anirudha (hop 1)` in Bubbu's terminal, then `Relaying ... (hop 2)` in
  Cairi's, and finally `Message completed the ring after 3 hop(s)` back in
  Anirudha's own terminal — the message visibly hops through every
  intermediate node instead of taking a direct link, just like a
  token-ring network.
- `Ctrl+C` Bubbu → Anirudha's `Successor` link and Cairi's `Predecessor` link
  both drop, breaking the loop at that one point — shows how a ring is
  vulnerable to any single node failure (unless you have a redundant
  counter-rotating ring, which real Token Ring/FDDI networks sometimes
  add).

---

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
