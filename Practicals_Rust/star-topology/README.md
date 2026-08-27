# Star Topology Demos — Computer Networks Practical


| Topology | Who sees the full topology? | How does data travel? | Single point of failure |
|---|---|---|---|
| **Star** | Only the hub | Hub relays between spokes | The hub — kill it, every spoke loses its only link |

compiled and run in the practical environment to confirm
this behavior (connections forming live, messages relaying correctly,
topology views updating on connect/disconnect).

---

## 1. Star Topology

One **hub** process, N **spoke** processes. Spokes only ever talk to the
hub; the hub relays messages between them.

**Terminal 1 — Hub** (list the names of spokes you expect):
```powershell
cargo run --bin star -- Hub hub 8000 Bubbu,Cairi
```

**Terminal 2 — Spoke Bubbu:**
```powershell
cargo run --bin star -- Bubbu spoke 127.0.0.1:8000
```

**Terminal 3 — Spoke Cairi:**
```powershell
cargo run --bin star -- Cairi spoke 127.0.0.1:8000
```

**What to observe:**
- The **hub's** screen redraws live with a full table (`Bubbu: CONNECTED`,
  `Cairi: CONNECTED`, ...).
- Each **spoke's** screen only ever shows its own single line: `Link to
  Hub: CONNECTED`. It has no idea the other spoke exists — exactly how a
  real switch/hub port isolates leaf devices.
- Type a message in Bubbu's terminal → it's relayed through the hub and
  appears in Cairi's terminal.
- `Ctrl+C` the Hub → **both** spokes immediately go `DOWN` — demonstrates
  the classic star single-point-of-failure.

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
