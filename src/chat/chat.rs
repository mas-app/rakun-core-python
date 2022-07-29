use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
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
use libp2p::floodsub::Topic;
use libp2p::noise::{AuthenticKeypair, X25519Spec};
use libp2p_tcp::{GenTcpConfig, GenTcpTransport};
use crate::chat::behaviour::AppBehaviour;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::chat::message::Command;

pub enum AgentStatus {
    CREATED,
    INITIALIZED,
    STOPPED,
}

pub struct AgentService {
    name: Cell<String>,
    coordinator: RefCell<Option<String>>,
    status: Cell<AgentStatus>,
    noise_keys: AuthenticKeypair<X25519Spec>,
    peer_id: PeerId,
    topic: Topic,
    rx: Arc<Mutex<Receiver<Command>>>,
    tx: Mutex<Sender<Command>>,
}


// fn worker(shared_rx: Arc<Mutex<Receiver<Command>>>) {
//     thread::spawn(move || loop {
//         {
//             if let Ok(mut rx) = shared_rx.lock() {
//                 match rx.try_recv() {
//                     Ok(_n) => {
//                          swarm.behaviour_mut().flood_sub.publish(topic.clone(), key.as_bytes().to_vec());
//                     }
//                     Err(_e) => {
//                         println!("No command received");
//                     }
//                 }
//             }
//         }
//     });
// }

impl AgentService {
    fn init(&self) -> Result<(), Box<dyn Error>> {
        println!("Chat Initializing...");
        self.status.set(AgentStatus::INITIALIZED);
        let topic = self.topic.clone();
        let noise_key = self.noise_keys.clone();
        let peer_id = self.peer_id.clone();
        let coordinator = self.coordinator.borrow().clone();
        let shared_rx = self.rx.clone();


        let transport = TokioTcpTransport::new(GenTcpConfig::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_key).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();
        let mut swarm = {
            let mdns = tokio::runtime::Runtime::new().unwrap().block_on(Mdns::new(Default::default())).unwrap();
            // let mdns = Mdns::new(Default::default()).await.unwrap();
            let mut behaviour = AppBehaviour {
                flood_sub: Floodsub::new(peer_id),
                mdns,
            };

            behaviour.flood_sub.subscribe(topic.clone());
            //
            SwarmBuilder::new(transport, behaviour, peer_id)
                // We want the connection background tasks to be spawned
                // onto the tokio runtime.
                .executor(Box::new(|fut| {
                    tokio::spawn(fut);
                }))
                .build()
        };
        // Reach out to another node if specified
        if let Some(to_dial) = coordinator {
            let addr: Multiaddr = to_dial.parse().unwrap();
            swarm.dial(addr).unwrap();
            println!("Dialed {:?}", to_dial);
        }

        thread::spawn(move || loop {
            {
                if let Ok(mut rx) = shared_rx.lock() {
                    match rx.try_recv() {
                        Ok(msg) => {
                            match msg {
                                Command::Get { key } => {
                                    swarm.behaviour_mut().flood_sub.publish(topic.clone(), key.as_bytes().to_vec());
                                }
                                Command::Set { key, val } => {
                                    println!("Publishing key: {}", key);
                                    // swarm.behaviour_mut().flood_sub.publish(topic.clone(), val.as_bytes().to_vec());
                                }
                                Command::None => {
                                    println!("No command received");
                                }
                            }
                            // swarm.behaviour_mut().flood_sub.publish(topic.clone(), key.as_bytes().to_vec());
                        }
                        Err(_e) => {
                            println!("No command received");
                        }
                    }
                }
            }
        });
        Ok(())
    }
}

impl AgentService {
    pub fn new(agent_name: String, address: Option<String>) -> AgentService {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&id_keys)
            .expect("Signing libp2p-noise static DH keypair failed.");

        let flood_sub_topic = Topic::new(agent_name.clone());
        let (tx, mut rx) = mpsc::channel(10);

        let agent_service = AgentService {
            name: Cell::new(agent_name),
            status: Cell::new(AgentStatus::CREATED),
            coordinator: RefCell::new(address),
            noise_keys,
            peer_id,
            topic: flood_sub_topic,
            rx: Arc::new(Mutex::new(rx)),
            tx: Mutex::new(tx),
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