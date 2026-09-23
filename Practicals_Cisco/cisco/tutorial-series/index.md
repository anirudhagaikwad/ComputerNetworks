# CISCO Packet Tracer Tutorial Series — Index

This is the full index of the hands-on Cisco Packet Tracer tutorial series. Each tutorial builds on the ones before it, gradually introducing more complex networking concepts and configurations — from a two-PC LAN all the way up to inter-AS routing with BGP.

Find the CISCO pkt files in the repo -

[![Repo](https://img.shields.io/badge/GitHub-CISCO--Packet--Tracer--Files-purple?logo=github)](https://github.com/anirudhagaikwad/ComputerNetworks/tree/main/Practicals_Cisco/cisco)

---

## Getting Started

| # | Tutorial | What it covers |
|---|----------|-----------------|
| 1 | [Basic Networking](tutorial1.md) | Two PCs via a switch, IPv4/IPv6 addressing, `ping` |
| 2 | [Configuring Multi-Switch Communication](tutorial2.md) | Two PCs across two switches, port activation, MAC learning |
| 3 | [Exploring Simulation Mode in Packet Tracer](tutorial3.md) | Watching ARP, pings, and frame forwarding step by step |

## VLANs, ARP & MAC Addressing

| # | Tutorial | What it covers |
|---|----------|-----------------|
| 4 | [Inter-VLAN Routing (Router-on-a-Stick)](tutorial4.md) | ROAS inter-VLAN routing using subinterfaces |
| 5 | [Understanding ARP in Packet Tracer](tutorial5.md) | How ARP resolves IPs to MAC addresses |
| 6 | [Understanding MAC Addresses in Packet Tracer](tutorial6.md) | How switches learn and forward on MAC addresses |

## Wireless Networking

| # | Tutorial | What it covers |
|---|----------|-----------------|
| 7 | [Setting Up a Wireless Network in Packet Tracer](tutorial7.md) | WRT300N wireless router, Wi-Fi + wired clients |
| 8 | [Adding a Wireless Access Point to a Network](tutorial8.md) | Adding an AccessPoint-PT to an existing VLAN |

## Routing Protocols

| # | Tutorial | What it covers |
|---|----------|-----------------|
| 9 | [Configuring Static Routing in Packet Tracer](tutorial9.md) | Manually defined routes on a three-router network |
| 10 | [Configuring RIP Routing in Packet Tracer](tutorial10.md) | RIPv1 — classful dynamic distance-vector routing |
| 15 | [Configuring RIPv2 (Classless) Routing in Packet Tracer](tutorial15.md) | RIPv2 — classless, VLSM-capable RIP |
| 11 | [Configuring OSPF Routing in Packet Tracer](tutorial11.md) | Link-state routing with areas and cost metrics |
| 12 | [Configuring EIGRP Routing in Packet Tracer](tutorial12.md) | Cisco's hybrid distance-vector protocol |
| 16 | [Configuring BGP Routing in Packet Tracer](tutorial16.md) | Inter-AS routing with eBGP |
| 13 | [Configuring a Default Route (Gateway of Last Resort)](tutorial13.md) | Static default routing for stub networks |

## Network Services

| # | Tutorial | What it covers |
|---|----------|-----------------|
| 14 | [DHCP Server Configuration in Cisco Packet Tracer](tutorial14.md) | Dynamic IP assignment via a DHCP server |

---

## Routing Protocol Coverage

A quick answer to "which routing protocols does this series actually configure":

| Protocol | Covered | Tutorial |
|---|---|---|
| Static routing | ✅ | [Tutorial 9](tutorial9.md) |
| Default route / gateway of last resort | ✅ | [Tutorial 13](tutorial13.md) |
| RIPv1 | ✅ | [Tutorial 10](tutorial10.md) |
| RIPv2 | ✅ *(added)* | [Tutorial 15](tutorial15.md) |
| OSPF | ✅ | [Tutorial 11](tutorial11.md) |
| EIGRP | ✅ | [Tutorial 12](tutorial12.md) |
| BGP | ✅ *(added)* | [Tutorial 16](tutorial16.md) |
| IS-IS | ❌ Not covered | — |

Tutorials 9–12 previously only cross-linked each other; Tutorials 15 and 16 have been added to that same "if you're after a different routing protocol" list so the series stays fully cross-referenced.
