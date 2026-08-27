# Bus Topology Demos — Computer Networks Practical

| Topology | Who sees the full topology? | How does data travel? | Single point of failure |
|---|---|---|---|
| **Bus**  | Everybody — the segment broadcasts full status to all stations | One hop, direct broadcast to every other station | The shared segment (models the cable) |

compiled and run in the practical environment to confirm
this behavior (connections forming live, messages relaying correctly,
topology views updating on connect/disconnect).

---
## Bus Topology

All stations tap into one shared **segment** process (standing in for the
shared cable). Unlike the star hub, the segment doesn't just relay
messages — it continuously **broadcasts the full connected-station list to
every station**, because a real bus is one shared broadcast domain that
every tap can observe.

```powershell
# Terminal 1 — the shared segment (list expected station names)
cargo run -- segment 6000 Anirudha,Bablya,Cairi

# Terminal 2
cargo run -- Anirudha station 127.0.0.1:6000

# Terminal 3
cargo run -- Bablya station 127.0.0.1:6000

# Terminal 4
cargo run -- Cairi station 127.0.0.1:6000
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
