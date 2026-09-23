# 15 – Configuring RIPv2 (Classless) Routing in Packet Tracer

This tutorial is the fifteenth in our Cisco Packet Tracer series and focuses on **RIPv2 (Routing Information Protocol version 2)**. [Tutorial 10](../tutorial-series/tutorial10.md) configured **RIPv1**, which is a **classful** protocol — it does not send subnet mask information in its updates. RIPv2 is **classless**: it carries the subnet mask with every advertised route, which means it supports **VLSM (Variable-Length Subnet Masking)** and discontiguous networks.

To make that difference concrete rather than just theoretical, we'll reuse the familiar three-router topology from Tutorials 9–12, but this time we'll carve all of the addressing out of a **single Class C network using VLSM** instead of three separate `/24`s. This is exactly the kind of addressing scheme that breaks under RIPv1 and works cleanly under RIPv2.

```{admonition} RIPv1 vs RIPv2 — Quick Comparison
:class: note
| | RIPv1 | RIPv2 |
|---|---|---|
| Classful / Classless | Classful | Classless |
| Subnet mask in updates | No | Yes |
| Supports VLSM | No | Yes |
| Update delivery | Broadcast (255.255.255.255) | Multicast (224.0.0.9) |
| Authentication | Not supported | Supported (plain text / MD5) |
| Auto-summarization | Always on | Configurable (`no auto-summary`) |
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

* **Three routers (R0, R1, R2)** connected in a linear series
* **Three switches (S0, S1, S2)** – one per router
* **Two PCs per switch** (6 total PCs)

Unlike Tutorial 10, all addressing here comes from **one classful network, `192.168.20.0/24`**, subnetted with VLSM so the LANs and the WAN links use *different* mask lengths.

![Figure](../../img/cisco-tutorials/tutorial-15/fig1.png)

---

## Part 2 – Device Placement and Cabling

### Step 2.1 – Add Devices to the Workspace

From **Network Devices** and **End Devices**, place:

* **3 Routers** (Router-PT-Empty)
* **3 Switches** (2960)
* **6 PCs**

Label the devices:

* Routers: **R0**, **R1**, **R2**
* Switches: **S0**, **S1**, **S2**
* PCs: **PC0–PC5**

### Step 2.2 – Add Network Modules to Routers

For this topology, use **Router-PT-Empty** devices. Each router needs **two Serial** and **two FastEthernet** interfaces.

```{admonition} Note
:class: note
If you still have the topology saved from Tutorial 9/10, you can reuse the same physical build — only the IP addressing and the routing configuration change in this tutorial.
```

Follow these steps for **R0**, **R1**, and **R2**:

1. Click the router to open its configuration window.
2. Go to the **Physical** tab.
3. Click the **power button** to turn off the router (the green light will go out).
4. In the module area, locate **PT-ROUTER-NM-1S** (Serial Port) and **PT-ROUTER-NM-1CFE** (FastEthernet).
5. Drag and insert **two** PT-ROUTER-NM-1S modules into the first two empty slots (from right to left).
6. Drag and insert **two** PT-ROUTER-NM-1CFE modules into the next two empty slots.
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
| PC4  | S2 | fa0/1           |
| PC5  | S2 | fa0/2           |
| S2   | R2 | fa0/24 → fa2/0  |

#### **Serial DTE Connections**

| From | To | Port/Interface |
|------|----|-----------------|
| R0   | R1 | se0/0 ↔ se1/0   |
| R1   | R2 | se0/0 ↔ se1/0   |

![Figure](../../img/cisco-tutorials/tutorial-15/fig2.png)

---

## Part 3 – IP Addressing Scheme (VLSM)

We'll subnet `192.168.20.0/24` using VLSM: `/26` blocks for the three LANs, and `/30` blocks carved out of the remainder for the two point-to-point WAN links.

### Subnet Allocation

| Subnet              | Devices      | Subnet Mask         |
|----------------------|--------------|----------------------|
| 192.168.20.0/26      | PC0, PC1, R0 | 255.255.255.192      |
| 192.168.20.64/26     | PC2, PC3, R1 | 255.255.255.192      |
| 192.168.20.128/26    | PC4, PC5, R2 | 255.255.255.192      |
| 192.168.20.192/30    | R0 ↔ R1      | 255.255.255.252      |
| 192.168.20.196/30    | R1 ↔ R2      | 255.255.255.252      |

```{admonition} Why this breaks RIPv1
:class: warning
RIPv1 does not transmit a subnet mask with its route entries — it assumes every subnet of a major network shares the mask of the interface the update was learned on. Here the LANs use `/26` and the WAN links use `/30`, all within the *same* classful network `192.168.20.0`. A RIPv1 router receiving these updates cannot tell a `/26` route apart from a `/30` route, so it miscalculates the network boundaries and connectivity breaks. RIPv2 fixes this by sending the actual subnet mask alongside each route, so mixed mask lengths within one major network (VLSM) work correctly.
```

### Step 3.1 – Assign IPs to PCs

Go to **Desktop > IP Configuration** on each PC:

| PC  | IP Address     | Subnet Mask     | Default Gateway |
|-----|----------------|------------------|-------------------|
| PC0 | 192.168.20.10  | 255.255.255.192  | 192.168.20.1      |
| PC1 | 192.168.20.11  | 255.255.255.192  | 192.168.20.1      |
| PC2 | 192.168.20.70  | 255.255.255.192  | 192.168.20.65     |
| PC3 | 192.168.20.71  | 255.255.255.192  | 192.168.20.65     |
| PC4 | 192.168.20.140 | 255.255.255.192  | 192.168.20.129    |
| PC5 | 192.168.20.141 | 255.255.255.192  | 192.168.20.129    |

![Figure](../../img/cisco-tutorials/tutorial-15/fig3.png)

```{admonition} Important
:class: important
Save your Packet Tracer file before configuring the routers — as before, this addressing scheme is reused if you want to experiment further.
```

---

## Part 4 – Router Configuration

Each router handles two types of connections:

- LAN-side via FastEthernet2/0, connected to a local switch
- WAN-side via Serial interfaces, connected to neighbouring routers
- All routers will be configured with **RIPv2**, with auto-summarization disabled

```{admonition} Note
:class: note
The RIPv2 configuration is performed using the following commands:

- `router rip` enters RIP configuration mode.
- `version 2` switches the process from RIPv1 to RIPv2.
- `no auto-summary` stops the router from automatically collapsing routes to their classful boundary at major network edges — required here so the `/26` and `/30` subnets are advertised with their real masks instead of being summarized into `192.168.20.0/24`.
- `network 192.168.20.0` is a single classful statement — since every interface on every router falls inside `192.168.20.0/24`, this one line is enough to enable RIP on all of a router's interfaces.
```

### Step 4.1 – R0 Configuration

```bash
enable
configure terminal
hostname R0

interface fa2/0
ip address 192.168.20.1 255.255.255.192
no shutdown
exit

interface se0/0
ip address 192.168.20.193 255.255.255.252
clock rate 64000
no shutdown
exit

router rip
version 2
no auto-summary
network 192.168.20.0
exit

write memory
exit
```

### Step 4.2 – R1 Configuration

```bash
enable
configure terminal
hostname R1

interface fa2/0
ip address 192.168.20.65 255.255.255.192
no shutdown
exit

interface se1/0
ip address 192.168.20.194 255.255.255.252
no shutdown
exit

interface se0/0
ip address 192.168.20.197 255.255.255.252
clock rate 64000
no shutdown
exit

router rip
version 2
no auto-summary
network 192.168.20.0
exit

write memory
exit
```

### Step 4.3 – R2 Configuration

```bash
enable
configure terminal
hostname R2

interface fa2/0
ip address 192.168.20.129 255.255.255.192
no shutdown
exit

interface se1/0
ip address 192.168.20.198 255.255.255.252
no shutdown
exit

router rip
version 2
no auto-summary
network 192.168.20.0
exit

write memory
exit
```

![Figure](../../img/cisco-tutorials/tutorial-15/fig4.png)

---

## Part 5 – Verification and Testing

### Step 5.1 – Confirm RIP Is Running Version 2

```bash
show ip protocols
```

Check that the output reads `Sending updates` / `Routing for Networks` with **"Sending version 2, Receiving version 2"** — if it still says version 1, double check the `version 2` line was entered inside `router rip` mode on every router.

![Figure](../../img/cisco-tutorials/tutorial-15/fig5.png)

### Step 5.2 – Check Routing Tables

```bash
show ip route
```

You should see RIP routes (`R`) to all remote networks, each showing its **correct mask** (`/26` for LANs, `/30` for WAN links) rather than being collapsed into a single `/24`:

```bash
show ip route rip
```

![Figure](../../img/cisco-tutorials/tutorial-15/fig6.png)

### Step 5.3 – Test Connectivity

From **PC0**, run:

```bash
ping 192.168.20.70
ping 192.168.20.140
```

From **PC3**, ping **PC4**:

```bash
ping 192.168.20.140
```

![Figure](../../img/cisco-tutorials/tutorial-15/fig7.png)

Repeat pings between any devices across networks.

---

## Summary

In this tutorial, you:

* Rebuilt the three-router, three-switch network with six PCs
* Subnetted a single classful network with VLSM (mixed `/26` and `/30` masks)
* Learned why this addressing scheme fails under RIPv1
* Configured RIPv2 with `no auto-summary` and verified correct, classless routing
