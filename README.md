# 🕸️ SabiSwarm (Vertex Swarm Challenge 2026)
**Zero-Cloud, Peer-to-Peer AI Mesh Network built on `libp2p`.**

Most AI agents are heavily dependent on centralized cloud infrastructure. SabiSwarm is a Proof of Concept for the "Ghost in the Machine" track, demonstrating how autonomous agents can coordinate, discover each other, and maintain state entirely off the grid.

## 🏗️ The V1 Architecture (Current PoC)
* **Core Engine:** Rust (`tokio` async runtime).
* **Networking Protocol:** `libp2p` v0.53.
* **Discovery:** Multicast DNS (mDNS) for zero-configuration local peer discovery.
* **Transport:** TCP multiplexed via Yamux and secured mathematically with Noise encryption.

Currently, SabiSwarm agents autonomously boot, generate unique ed25519 cryptographic identities, and scan the local grid to establish direct P2P handshakes without relying on a central DNS or cloud broker.

## 🚀 The V2 Roadmap (State & Consensus)
This PoC lays the network foundation. The immediate next steps for production are:
1. **Gossipsub Protocol:** Implementing `libp2p-gossipsub` so agents can publish and subscribe to offline state updates (e.g., sharing a localized blockchain state or task queue).
2. **Distributed Hash Table (Kademlia DHT):** Allowing the swarm to securely store and retrieve payload data across the local devices, ensuring that if one node goes down, the swarm retains the memory.
3. **Local LLM Execution:** Hooking the Rust networking daemon into a locally quantized model (like Gemma), allowing the swarm to process data collaboratively.

## 🛠️ Local Quickstart
Run this on two separate terminal windows on the same machine (or on two machines sharing the same local network) to see autonomous discovery in action:
```bash
cargo run
