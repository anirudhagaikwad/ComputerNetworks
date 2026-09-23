# 16 – Configuring BGP Routing in Packet Tracer

This tutorial is the sixteenth in our Cisco Packet Tracer series and focuses on **BGP (Border Gateway Protocol)**. Every routing protocol earlier in this series — RIP, OSPF, and EIGRP — is an **IGP (Interior Gateway Protocol)**, designed to route traffic *within* a single organization's network. BGP is an **EGP (Exterior Gateway Protocol)**: it's the protocol that routes traffic *between* separate organizations, each identified by its own **Autonomous System (AS)** number. It's the protocol that holds the internet itself together.

Because BGP only makes sense between two different autonomous systems, this tutorial uses a smaller, two-AS topology rather than extending the three-router network from Tutorials 9–12, 15.

```{admonition} eBGP vs iBGP
:class: note
BGP peerings come in two flavours:
- **eBGP (External BGP)** — between routers in *different* AS numbers, typically directly connected. This is what we configure below.
- **iBGP (Internal BGP)** — between routers *inside the same* AS, used to carry BGP-learned routes across an organization's own network. iBGP requires extra considerations (like full-mesh peering or route reflectors) that are outside the scope of this tutorial.
```

If you're after a different routing protocol, check out -

- [Tutorial 9: Configuring Static Routing in Packet Tracer](../tutorial-series/tutorial9.md)
- [Tutorial 10: Configuring RIP Routing in Packet Tracer](../tutorial-series/tutorial10.md)
- [Tutorial 11: Configuring OSPF Routing in Packet Tracer](../tutorial-series/tutorial11.md)
- [Tutorial 12: Configuring EIGRP Routing in Packet Tracer](../tutorial-series/tutorial12.md)
- [Tutorial 15: Configuring RIPv2 (Classless) Routing in Packet Tracer](../tutorial-series/tutorial15.md)
- [Tutorial 16: Configuring BGP Routing in Packet Tracer](../tutorial-series/tutorial16.md)

Find the CISCO pkt files in the repo -

[![Repo](https://img.shields.io/badge/GitHub-CISCO--Packet--Tracer--Files-purple?logo=github)](https://github.com/anirudhagaikwad/ComputerNetworks/tree/main/Practicals_Cisco/cisco)

---

## Part 1 – Network Topology Overview

This network includes:

* **Two routers**, each representing a different organization:
  * **R0** in **AS 100**
  * **R1** in **AS 200**
* **One switch per router** (S0, S1)
* **Two PCs per switch** (4 total PCs)
* A single **serial link** directly connecting R0 and R1 — this is the **eBGP peering link**

The goal is for PCs in AS 100 to reach PCs in AS 200 (and vice-versa) purely through BGP-advertised routes.

![Figure](../../img/cisco-tutorials/tutorial-16/fig1.png)

---

## Part 2 – Device Placement and Cabling

### Step 2.1 – Add Devices to the Workspace

From **Network Devices** and **End Devices**, place:

* **2 Routers** (Router-PT-Empty)
* **2 Switches** (2960)
* **4 PCs**

Label the devices:

* Routers: **R0** (AS 100), **R1** (AS 200)
* Switches: **S0**, **S1**
* PCs: **PC0–PC3**

### Step 2.2 – Add Network Modules to Routers

Each router needs **one Serial** and **one FastEthernet** interface.

Follow these steps for **R0** and **R1**:

1. Click the router to open its configuration window.
2. Go to the **Physical** tab.
3. Click the **power button** to turn off the router.
4. In the module area, locate **PT-ROUTER-NM-1S** (Serial Port) and **PT-ROUTER-NM-1CFE** (FastEthernet).
5. Drag and insert **one** PT-ROUTER-NM-1S module into an empty slot.
6. Drag and insert **one** PT-ROUTER-NM-1CFE module into an empty slot.
7. Click the **power button** again to turn the router back on.

### Step 2.3 – Cabling

#### **Copper Straight-Through Connections**

| From | To | Port/Interface |
|------|----|-----------------|
| PC0  | S0 | fa0/1           |
| PC1  | S0 | fa0/2           |
| S0   | R0 | fa0/24 → fa2/0  |
| PC2  | S1 | fa0/1           |
| PC3  | S1 | fa0/2           |
| S1   | R1 | fa0/24 → fa2/0  |

#### **Serial DTE Connection**

| From | To | Port/Interface |
|------|----|-----------------|
| R0   | R1 | se0/0 ↔ se1/0   |

![Figure](../../img/cisco-tutorials/tutorial-16/fig2.png)

---

## Part 3 – IP Addressing Scheme

| Subnet             | Devices           | Subnet Mask       |
|----------------------|--------------------|----------------------|
| 172.16.1.0/24        | PC0, PC1, R0       | 255.255.255.0        |
| 172.16.2.0/24        | PC2, PC3, R1       | 255.255.255.0        |
| 203.0.113.0/30       | R0 ↔ R1 (eBGP link)| 255.255.255.252      |

### Step 3.1 – Assign IPs to PCs

Go to **Desktop > IP Configuration** on each PC:

| PC  | IP Address   | Subnet Mask   | Default Gateway |
|-----|--------------|-----------------|-------------------|
| PC0 | 172.16.1.10  | 255.255.255.0   | 172.16.1.1        |
| PC1 | 172.16.1.11  | 255.255.255.0   | 172.16.1.1        |
| PC2 | 172.16.2.10  | 255.255.255.0   | 172.16.2.1        |
| PC3 | 172.16.2.11  | 255.255.255.0   | 172.16.2.1        |

![Figure](../../img/cisco-tutorials/tutorial-16/fig3.png)

```{admonition} Important
:class: important
Save your Packet Tracer file before configuring the routers — you'll want this baseline if you go on to experiment with route filtering or a third AS later.
```

---

## Part 4 – Router Configuration

Each router advertises its own LAN into BGP and peers with the router in the other AS across the serial link.

```{admonition} Note
:class: note
The BGP configuration is performed using the following commands:

- `router bgp <AS-number>` starts the BGP process for the router's **own** AS. Unlike RIP/OSPF/EIGRP, this does not enable BGP on any interface by itself.
- `neighbor <remote-ip> remote-as <remote-AS-number>` defines a peer to exchange routes with. Because the neighbor's AS number differs from the router's own, Packet Tracer/IOS treats this as an **eBGP** peering.
- `network <network> mask <subnet-mask>` tells BGP which locally-owned prefix to originate and advertise to peers. The prefix must already exist in the router's own routing table (e.g. as a directly connected network) — BGP does not discover it automatically the way an IGP would.
```

### Step 4.1 – R0 Configuration (AS 100)

```bash
enable
configure terminal
hostname R0

interface fa2/0
ip address 172.16.1.1 255.255.255.0
no shutdown
exit

interface se0/0
ip address 203.0.113.1 255.255.255.252
clock rate 64000
no shutdown
exit

router bgp 100
neighbor 203.0.113.2 remote-as 200
network 172.16.1.0 mask 255.255.255.0
exit

write memory
exit
```

### Step 4.2 – R1 Configuration (AS 200)

```bash
enable
configure terminal
hostname R1

interface fa2/0
ip address 172.16.2.1 255.255.255.0
no shutdown
exit

interface se1/0
ip address 203.0.113.2 255.255.255.252
no shutdown
exit

router bgp 200
neighbor 203.0.113.1 remote-as 100
network 172.16.2.0 mask 255.255.255.0
exit

write memory
exit
```

![Figure](../../img/cisco-tutorials/tutorial-16/fig4.png)

---

## Part 5 – Verification and Testing

### Step 5.1 – Check the BGP Neighbor State

```bash
show ip bgp summary
```

Look at the neighbor's `State/PfxRcd` column — a numeric value (e.g. `1`) means the peering is **Established** and a prefix has been received. If it instead shows `Idle` or `Active`, the peering hasn't come up yet — double check the serial link's IPs, the AS numbers, and that both interfaces show `no shutdown`.

![Figure](../../img/cisco-tutorials/tutorial-16/fig5.png)

### Step 5.2 – Inspect the BGP Table

```bash
show ip bgp
```

You should see both `172.16.1.0/24` and `172.16.2.0/24` listed, each with its next-hop and AS path.

![Figure](../../img/cisco-tutorials/tutorial-16/fig6.png)

### Step 5.3 – Check Routing Tables

```bash
show ip route bgp
```

You should see the remote LAN as a BGP route (`B`), learned entirely through the eBGP peering rather than a static entry or an IGP.

![Figure](../../img/cisco-tutorials/tutorial-16/fig7.png)

### Step 5.4 – Test Connectivity

From **PC0**, run:

```bash
ping 172.16.2.10
```

From **PC2**, run:

```bash
ping 172.16.1.10
```

![Figure](../../img/cisco-tutorials/tutorial-16/fig8.png)

---

## Summary

In this tutorial, you:

* Built a two-AS topology (AS 100 and AS 200) connected by a single eBGP link
* Learned how BGP differs from the IGPs (RIP, OSPF, EIGRP) covered earlier in this series
* Configured `router bgp`, `neighbor ... remote-as`, and `network ... mask` on both routers
* Verified the eBGP peering state and confirmed end-to-end connectivity across autonomous systems
