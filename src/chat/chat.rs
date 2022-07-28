use std::cell::Cell;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    noise,
    swarm::{dial_opts::DialOpts, NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    // `TokioTcpTransport` is available through the `tcp-tokio` feature.
    tcp::TokioTcpTransport,
    Multiaddr,
    NetworkBehaviour,
    PeerId,
    Transport,
};
use libp2p::core::muxing;
use libp2p::core::transport::Boxed;
use libp2p::floodsub::Topic;
use libp2p::noise::{AuthenticKeypair, X25519Spec};
use libp2p_tcp::{GenTcpConfig, GenTcpTransport};

pub enum AgentStatus {
    CREATED,
    INITIALIZED,
    STOPPED,
}

pub struct AgentService {
    name: Cell<String>,
    status: Cell<AgentStatus>,
    noise_keys: Cell<AuthenticKeypair<X25519Spec>>,
    flood_sub: Cell<Topic>,
}


impl AgentService {
    fn init(&self) {
        println!("Chat Initializing...");
        self.status.set(AgentStatus::INITIALIZED);
    }
}

impl AgentService {
    pub fn new(agent_name: String) -> AgentService {
        let id_keys = identity::Keypair::generate_ed25519();
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&id_keys)
            .expect("Signing libp2p-noise static DH keypair failed.");

        let flood_sub_topic = floodsub::Topic::new(agent_name.clone());

        let mut agent_service = AgentService {
            name: Cell::new(agent_name),
            status: Cell::new(AgentStatus::CREATED),
            noise_keys: Cell::new(noise_keys),
            flood_sub: Cell::new(flood_sub_topic),
        };
        agent_service.init();
        agent_service
    }

    pub fn start(&self) {
        println!("AgentService started");
    }

    pub fn stop(&self) {
        self.status.set(AgentStatus::STOPPED);
    }

    pub fn chat(&self, message: &str) {
        println!("AgentService received message: {}", message);
    }
}