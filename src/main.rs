use libp2p::{
    futures::StreamExt, // Crucial for select_next_some()
    mdns, 
    noise, 
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, 
    yamux,
};
use std::error::Error;

// ==========================================
// 1. THE BRAIN OF THE AGENT
// ==========================================
#[derive(NetworkBehaviour)]
struct SabiSwarmBehaviour {
    mdns: mdns::tokio::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the logger so we can see the mesh network events
    tracing_subscriber::fmt::init();
    println!("🚀 Booting SabiSwarm Offline P2P Node...");

    // ==========================================
    // 2. CRYPTOGRAPHIC IDENTITY
    // ==========================================
    // Every node in the swarm needs a unique, mathematically verifiable ID.
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = libp2p::PeerId::from(local_key.public());
    println!("🤖 Local Agent Identity: {local_peer_id}");

    // ==========================================
    // 3. THE SWARM BUILDER (v0.53 API)
    // ==========================================
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let peer_id = libp2p::PeerId::from(key.public());
            
            // Initialize the mDNS behaviour to scan for local peers.
            // We use .expect() here to handle errors internally, 
            // allowing us to return the raw struct cleanly.
            let mdns_behaviour = mdns::tokio::Behaviour::new(
                mdns::Config::default(), 
                peer_id
            ).expect("CRITICAL: Failed to initialize mDNS");

            // Return the behaviour directly (No Ok() wrapper needed)
            SabiSwarmBehaviour { mdns: mdns_behaviour }
        })?
        .with_swarm_config(|cfg| cfg)
        .build();

    // ==========================================
    // 4. LISTEN FOR CONNECTIONS
    // ==========================================
    // "0.0.0.0:0" tells the OS to assign any available port dynamically.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    println!("🌐 SabiSwarm active. Scanning local grid for peers...");

    // ==========================================
    // 5. THE EVENT LOOP
    // ==========================================
    // The agent stays alive permanently, reacting to network events.
    loop {
        tokio::select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("✅ Listening on local address: {address}");
                }
                SwarmEvent::Behaviour(SabiSwarmBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        println!("🔗 [CONTACT] Discovered Peer Agent: {peer_id} at {multiaddr}");
                    }
                }
                SwarmEvent::Behaviour(SabiSwarmBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, multiaddr) in list {
                        println!("⚠️ [LOST] Peer Agent vanished: {peer_id} at {multiaddr}");
                    }
                }
                _ => {} // Ignore all other background events for now
            }
        }
    }
}