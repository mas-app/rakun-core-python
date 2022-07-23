use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::SwarmEvent;
use libp2p::{identity, Multiaddr, PeerId, Swarm};
use libp2p::futures::StreamExt;

pub async fn run_peer(address: Option<String>) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await.unwrap();

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    if let Some(addr) = address {
        let remote = addr.parse::<Multiaddr>().unwrap();
        swarm.dial(remote.clone()).unwrap();
        println!("Dialed {}", remote);
    } else {
        println!("Listening on all interfaces");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("New listen address: {}", address)
            }
            SwarmEvent::Behaviour(event) => println!("Behaviour event: {:?}", event),
            _ => {}
        }
    }
}