# Practical Lab: Understanding & Calculating Subnetting
### Network Used: 192.168.1.0 /24

---

## 1. What Is Subnetting?

Subnetting is the process of dividing one large IP network into several smaller networks (called **subnets**). Instead of wasting a huge block of addresses on one flat network, you split it up so that:

- Each department, floor, VLAN, or site gets its own logical network
- Broadcast traffic is contained within smaller segments (better performance)
- IP addresses are used more efficiently (less waste)
- Security and routing policies can be applied per subnet

Every IPv4 address has two parts:

| Part | Purpose |
|---|---|
| **Network portion** | Identifies which network the address belongs to |
| **Host portion** | Identifies a specific device within that network |

The **subnet mask** tells you where the network portion ends and the host portion begins.

---

## 2. Key Concepts You Need Before Calculating

### 2.1 CIDR Notation
CIDR notation (Classless Inter-Domain Routing) is a short way to write an IP address and its subnet mask. It uses an IP address, a slash (/), and a number. That number tells you how many bits at the start of the address belong to the network. For example, 192.168.1.0/24 means the first 24 bits are the network

`/24` means the first 24 bits of the 32-bit IPv4 address are the **network bits**. The remaining `32 - 24 = 8` bits are **host bits**.

### 2.2 Subnet Mask
`/24` = `255.255.255.0`

Binary breakdown of the mask:

```
11111111.11111111.11111111.00000000
   255   .   255   .   255   .   0
```

### 2.3 The Core Formulas

| Value | Formula | 
|---|---|
| Number of subnets created (when borrowing *b* bits) | 2^b |
| Number of usable hosts per subnet | 2^h − 2 (h = host bits) |
| Total addresses per subnet | 2^h |
| Broadcast address | Last address in the subnet range |
| Network address | First address in the subnet range |

> **Why subtract 2?** Every subnet reserves its **first address** (Network ID) and **last address** (Broadcast) — these cannot be assigned to a host.

---

## 3. Base Network Analysis: 192.168.1.0/24

| Property | Value |
|---|---|
| IP Address given | 192.168.1.1 |
| Network Address | 192.168.1.0 |
| Subnet Mask | 255.255.255.0 |
| CIDR | /24 |
| Host bits available | 8 (32 − 24) |
| Total addresses | 2^8 = 256 |
| Usable hosts | 256 − 2 = **254** |
| First usable host | 192.168.1.1 |
| Last usable host | 192.168.1.254 |
| Broadcast address | 192.168.1.255 |

So `192.168.1.1` is simply the **first usable host address** in the default /24 network — commonly assigned to a router/gateway.

---

## 4. LAB EXERCISE — Subnetting 192.168.1.0/24 into Smaller Networks

**Scenario:** You are the network admin for a small office. You have been given the block `192.168.1.0/24` and must divide it into **4 equal subnets** to serve 4 departments (HR, Sales, IT, Finance), each needing no more than 50 hosts.

### Step 1 — Determine bits needed for subnets
You need 4 subnets → 2^b ≥ 4 → b = 2 bits borrowed.

New prefix = /24 + 2 = **/26**

### Step 2 — Determine bits left for hosts
Host bits = 32 − 26 = 6 bits
Usable hosts per subnet = 2^6 − 2 = **62** (fits the 50-host requirement ✅)

### Step 3 — New Subnet Mask
```
/26 = 255.255.255.192
```
Binary: `11111111.11111111.11111111.11000000`

### Step 4 — Calculate the Block Size (Increment)
Block size = 256 − 192 = **64**
So each subnet jumps by 64 in the last octet.

### Step 5 — List All 4 Subnets

| Subnet # | Department | Network Address | Usable Host Range | Broadcast Address |
|---|---|---|---|---|
| 1 | HR | 192.168.1.0 /26 | 192.168.1.1 – 192.168.1.62 | 192.168.1.63 |
| 2 | Sales | 192.168.1.64 /26 | 192.168.1.65 – 192.168.1.126 | 192.168.1.127 |
| 3 | IT | 192.168.1.128 /26 | 192.168.1.129 – 192.168.1.190 | 192.168.1.191 |
| 4 | Finance | 192.168.1.192 /26 | 192.168.1.193 – 192.168.1.254 | 192.168.1.255 |

> Notice: `192.168.1.1` (your original IP) now falls in **Subnet 1 (HR)** as its first usable host / gateway address.

---

## 5. Manual Calculation Method (Step-by-Step, Reusable for Any /24)

Use this checklist for **any** future subnetting problem:

1. **Write the mask in binary.**
2. **Determine how many subnets you need** → find smallest `b` where `2^b ≥ required subnets`.
3. **New prefix = old prefix + b.**
4. **Host bits = 32 − new prefix.**
5. **Usable hosts = 2^(host bits) − 2.**
6. **Block size = 256 − (last non-255 octet value of new mask).**
7. **List subnets by adding the block size repeatedly to the relevant octet**, starting from the base network address.
8. For each subnet:
   - Network ID = first address
   - Broadcast = last address
   - Usable range = everything in between

---

## 6. Quick Reference Table — Common /24 Subnetting Breakdowns

| New Prefix | Mask | Borrowed Bits | # Subnets | Hosts/Subnet | Block Size |
|---|---|---|---|---|---|
| /25 | 255.255.255.128 | 1 | 2 | 126 | 128 |
| /26 | 255.255.255.192 | 2 | 4 | 62 | 64 |
| /27 | 255.255.255.224 | 3 | 8 | 30 | 32 |
| /28 | 255.255.255.240 | 4 | 16 | 14 | 16 |
| /29 | 255.255.255.248 | 5 | 32 | 6 | 8 |
| /30 | 255.255.255.252 | 6 | 64 | 2 | 4 |

---

## 7. Hands-On Practice Tasks (Try Before Checking Answers)

Using base network **192.168.1.0/24**, solve the following. Answers are at the bottom.

1. Subnet into **8 equal networks**. What is the new mask, and what is the 3rd subnet's usable range?
2. You need **exactly 30 hosts** per subnet for a branch office. What CIDR should you use, and how many subnets does it create?
3. Given host `192.168.1.130 /26`, what is its network address and broadcast address?

<details>
<summary>Click to reveal answers</summary>

**Q1:**
- 8 subnets → 2^3 = 8 → borrow 3 bits → new prefix = /27
- Mask = 255.255.255.224, block size = 32
- Subnets: .0, .32, .64, .96, .128, .160, .192, .224
- 3rd subnet (index starts at 1) = 192.168.1.64/27 → usable range: 192.168.1.65 – 192.168.1.94

**Q2:**
- Need 30 usable hosts → 2^h − 2 ≥ 30 → h = 5 (2^5−2=30)
- New prefix = 32 − 5 = /27
- Subnets created = 2^(27−24) = 8

**Q3:**
- /26 → block size 64 → subnets at .0, .64, .128, .192
- 130 falls between 128–191 → Network = 192.168.1.128, Broadcast = 192.168.1.191
- Usable range: 192.168.1.129 – 192.168.1.190

</details>

---

## 8. Lab Verification Commands (Optional — Real Device Practice)

If you want to verify your calculations on a real system:

**Linux/Mac:**
```bash
ipcalc 192.168.1.0/26
```

**Windows (PowerShell with subnet calculator module, or manual):**
```powershell
# Check current adapter's IP/mask
Get-NetIPAddress
```

**Cisco IOS (on a router/switch):**
```
Router(config)# interface g0/0
Router(config-if)# ip address 192.168.1.1 255.255.255.192
```

---

## 9. Summary Cheat Sheet

- **More borrowed bits → more subnets, fewer hosts per subnet.**
- **Fewer borrowed bits → fewer subnets, more hosts per subnet.**
- Always reserve **Network ID** and **Broadcast** — never assign them to a device.
- `/24` = 254 usable hosts. Every bit you borrow **halves** the hosts and **doubles** the subnets.
- Formula to memorize:
  ```
  Usable Hosts = 2^(32 - prefix) - 2
  Number of Subnets = 2^(new prefix - old prefix)
  ```

---

*End of Lab — Base network: 192.168.1.0/24 | Example host: 192.168.1.1*
