# netsniff

Network packet sniffer, inspect your computers traffic on a low level

## Prerequisites

- Npcap Runtime & SDK in C:\ which are available at https://npcap.com/#download

## Usage

Run the program and pick the device you want to listen on:
- Some things obviously hidden due to privacy

```
Pick a device: 
  0: Some("WAN Miniport (Network Monitor)")
  1: Some("WAN Miniport (IPv6)")
  2: Some("WAN Miniport (IP)")
  3: Some("Intel(R) Ethernet Controller (3) I225-V")
  4: Some("Adapter for loopback traffic capture")
3
[22:34:05.354172]: 
  [L2] Ethernet2 MAC DC:A6:32:CD:**:** -> C8:7F:**:**:**:** 
  [L3] Ipv4 192.168.**.*** -> 192.168.**.*** (TTL: 64)
  [L4] UDP PORT: 53 -> 58431
  [Data] Payload Length: 154 bytes 
[22:34:05.377702]: 
  [L2] Ethernet2 MAC 08:BF:B8:B0:**:** -> C8:7F:**:**:**:** 
  [L3] Ipv4 140.82.121.* -> 192.168.**.*** (TTL: 56)
  [L4] TCP PORT: 22 -> 56367 | Flags: ["SYN @ 2760198297"]
  [Data] Payload Length: 66 bytes 
[22:34:05.377766]: 
  [L2] Ethernet2 MAC C8:7F:**:**:**:** -> 08:BF:B8:B0:B3:A0 
  [L3] Ipv4 192.168.**.*** -> 140.82.121.* (TTL: 128)
  [L4] TCP PORT: 56367 -> 22 | Flags: ["ACK @ 2760198298"]
  [Data] Payload Length: 54 bytes 
```