# Ping: Complete Guide

## What is Ping?

**Ping** is a network diagnostic utility used to test whether a remote
host is reachable over an IP network. It also measures the Round-Trip
Time (RTT) for packets.

## Protocol Used

Ping uses the **Internet Control Message Protocol (ICMP)**. - IPv4: ICMP
Echo Request (Type 8) and Echo Reply (Type 0) - IPv6: ICMPv6 Echo
Request (Type 128) and Echo Reply (Type 129)

> ICMP is a network-layer protocol carried directly inside IP. It does
> **not** use TCP or UDP.

## How Ping Works

1.  Source creates an ICMP Echo Request.
2.  Packet is encapsulated in an IP packet.
3.  Routers forward it to the destination.
4.  Destination returns an ICMP Echo Reply.
5.  Sender measures RTT and reports packet loss.

## Typical Output

-   Reply from host
-   RTT (latency)
-   TTL/Hop Limit
-   Packet loss statistics

## Common Uses

-   Check connectivity
-   Measure latency
-   Detect packet loss
-   Basic troubleshooting

## Limitations

-   ICMP may be blocked by firewalls.
-   Successful ping does not guarantee an application/service is
    working.
-   Failed ping does not always mean the host is down.

## Alternatives to Ping

  Tool                   Protocol       Purpose
  ---------------------- -------------- ----------------------------------
  traceroute / tracert   ICMP/UDP/TCP   Shows network path
  mtr                    ICMP           Continuous ping + traceroute
  fping                  ICMP           Ping many hosts
  arping                 ARP            Test devices on local LAN
  tcping                 TCP            Check if a TCP port is reachable
  hping3                 TCP/UDP/ICMP   Advanced packet testing
  nping (Nmap)           TCP/UDP/ICMP   Flexible network testing
  curl                   HTTP/HTTPS     Test web servers
  nc (netcat)            TCP/UDP        Check ports/connectivity

## Ping vs TCP Ping

-   Ping: ICMP only.
-   TCP Ping: Uses TCP SYN or connect to test a specific port, useful
    when ICMP is blocked.

## Security Notes

-   Some networks disable ICMP to reduce scanning.
-   Ping can be used for monitoring and troubleshooting, but should not
    be the only health check.

## Summary

-   Ping uses **ICMP**, not TCP/UDP.
-   Measures reachability, latency, and packet loss.
-   Useful for basic diagnostics.
-   Alternatives include **traceroute, mtr, tcping, hping3, nping,
    arping, fping, curl, and netcat**.
