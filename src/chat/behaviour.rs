use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex,
    Multiaddr,
    NetworkBehaviour,
    // `TokioTcpTransport` is available through the `tcp-tokio` feature.
    noise,
    PeerId,
    swarm::{dial_opts::DialOpts, NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpTransport,
    Transport,
};
use libp2p::noise::{X25519Spec};
use libp2p_tcp::{GenTcpTransport};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct AppBehaviour {
    pub(crate) flood_sub: Floodsub,
    pub(crate) mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for AppBehaviour {
    // Called when `floodsub` produces an event.
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            println!(
                "Received: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for AppBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.flood_sub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.flood_sub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}