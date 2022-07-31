use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;

use bytes::Bytes;
use libp2p::{core::upgrade, floodsub::{self, Floodsub, FloodsubEvent}, identity, mdns::{Mdns, MdnsEvent}, mplex, Multiaddr, NetworkBehaviour, noise, PeerId, swarm::{dial_opts::DialOpts, NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent}, Swarm, tcp::TokioTcpTransport, Transport};
use libp2p::floodsub::Topic;
use libp2p::noise::{AuthenticKeypair, X25519Spec};
use libp2p_tcp::{GenTcpConfig, GenTcpTransport};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::chat::behaviour::AppBehaviour;
use crate::chat::message::Command;

pub enum AgentStatus {
    CREATED,
    INITIALIZED,
    STOPPED,
}

pub struct AgentService {
    name: Mutex<String>,
    coordinator: Mutex<Option<String>>,
    status: Mutex<Cell<AgentStatus>>,
    topic: Topic,
    rx: Arc<Mutex<Receiver<Command>>>,
    tx: Mutex<Sender<Command>>,
    swarm: Swarm<AppBehaviour>,
}

impl AgentService {
    fn init(&self) {
        println!("Chat Initializing...");

        // thread::spawn(move || {
        //     rt.block_on(async move {
        //         let mut swarm = swarm_shared.lock().unwrap();
        //         let er = swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap());
        //
        //         loop {
        //             tokio::select! {
        //                 event = swarm.borrow().clone().next() => {
        //                     let event = event.unwrap();
        //                     println!("{:?}", event);
        //                 },
        //                 command = self.command_receiver.next() => match command {
        //                     Some(c) => self.handle_command(c).await,
        //                     // Command channel closed, thus shutting down the network event loop.
        //                     None=>  return,
        //                 },
        //             }
        //         }
        //
        //
        //         // if er.is_err() {
        //         //     println!("Error: {:?}", er);
        //         // }
        //         // while let Some(msg) = shared_rx.lock().unwrap().recv().await {
        //         //     match msg {
        //         //         Command::Get { key } => {
        //         //             println!("Getting key: {}", key);
        //         //         }
        //         //         Command::Set { key, val } => {
        //         //             println!("Setting key: {} to val: {}", key, String::from_utf8_lossy(&val));
        //         //             swarm.behaviour_mut().flood_sub.publish(topic.clone(), val);
        //         //         }
        //         //         Command::None => {
        //         //             // println!("No command received");
        //         //         }
        //         //     }
        //         // }
        //     })
        // });
    }

    async fn waiting_for_message(&self, swarm_shared: Arc<Mutex<Swarm<AppBehaviour>>>) {
        let topic = self.topic.clone();
        let shared_rx = self.rx.clone();
        {
            let mut swarm = swarm_shared.lock().unwrap();
            let er = swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap());
            if er.is_err() {
                println!("Error: {:?}", er);
            }
            while let Some(msg) = shared_rx.lock().unwrap().recv().await {
                match msg {
                    Command::Get { key } => {
                        println!("Getting key: {}", key);
                    }
                    Command::Set { key, val } => {
                        swarm.behaviour_mut().flood_sub.publish(topic.clone(), val);
                        // swarm.behaviour_mut().flood_sub.publish(topic.clone(), val.as_bytes().to_vec());
                    }
                    Command::None => {
                        // println!("No command received");
                    }
                }
            }
        }
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

        let transport = TokioTcpTransport::new(GenTcpConfig::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        let mut swarm = {
            let mdns = tokio::runtime::Runtime::new().unwrap().block_on(Mdns::new(Default::default())).unwrap();
            // let mdns = Mdns::new(Default::default()).await.unwrap();
            let mut behaviour = AppBehaviour {
                flood_sub: Floodsub::new(peer_id),
                mdns,
            };

            behaviour.flood_sub.subscribe(flood_sub_topic.clone());
            //
            SwarmBuilder::new(transport, behaviour, peer_id)
                // We want the connection background tasks to be spawned
                // onto the tokio runtime.
                .executor(Box::new(|fut| {
                    tokio::spawn(fut);
                }))
                .build()
        };


        let (tx, mut rx) = mpsc::channel(0);


        let agent_service = AgentService {
            name: Mutex::new(agent_name),
            status: Mutex::new(Cell::new(AgentStatus::CREATED)),
            coordinator: Mutex::new(address),
            topic: flood_sub_topic,
            rx: Arc::new(Mutex::new(rx)),
            tx: Mutex::new(tx),
            swarm,
        };
        agent_service.init();
        agent_service
    }

    pub fn start(&self) {
        println!("AgentService started");
    }

    pub fn stop(&self) {
        self.status.lock().unwrap().set(AgentStatus::STOPPED);
    }

    pub fn chat(&self, message: String) {
        let mut tx = self.tx.lock().unwrap();
        let data = Arc::new(Mutex::new(Bytes::from(message.clone())));
        let rt = tokio::runtime::Runtime::new().unwrap();
        tokio::task::block_in_place(move || {
            rt.block_on(async move {
                let msg = Command::Set {
                    key: "DATA".to_string(),
                    val: data.lock().unwrap().clone(),
                };
                tx.send(msg).await.unwrap();
            });
        });
    }
}