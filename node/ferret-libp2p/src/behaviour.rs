use futures::Async;
use libp2p::core::identity::Keypair;
use libp2p::core::PeerId;
use libp2p::gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, Topic, TopicHash};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaConfig, KademliaEvent};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess};
use libp2p::tokio_io::{AsyncRead, AsyncWrite};
use libp2p::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent", poll_method = "poll")]
pub struct MyBehaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub gossipsub: Gossipsub<TSubstream>,
    pub mdns: Mdns<TSubstream>,
    pub kad: Kademlia<TSubstream, MemoryStore>,
    #[behaviour(ignore)]
    events: Vec<MyBehaviourEvent>,
}

pub enum MyBehaviourEvent {
    DiscoveredPeer(PeerId),
    ExpiredPeer(PeerId),
    GossipMessage {
        source: PeerId,
        topics: Vec<TopicHash>,
        message: Vec<u8>,
    },
}
impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<KademliaEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, event: KademliaEvent) {
        match event {
            KademliaEvent::BootstrapResult(_) => (),
            KademliaEvent::GetClosestPeersResult(_) => (),
            KademliaEvent::GetProvidersResult(_) => (),
            KademliaEvent::StartProvidingResult(_) => (),
            KademliaEvent::RepublishProviderResult(_) => (),
            KademliaEvent::GetRecordResult(_) => (),
            KademliaEvent::PutRecordResult(_) => (),
            KademliaEvent::RepublishRecordResult(_) => (),
            KademliaEvent::Discovered { peer_id, addresses, ty } => (),
            KademliaEvent::RoutingUpdated { peer, addresses, old_peer } => (),
            KademliaEvent::UnroutablePeer { peer } => (),
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<MdnsEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.events.push(MyBehaviourEvent::DiscoveredPeer(peer))
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.events.push(MyBehaviourEvent::ExpiredPeer(peer))
                    }
                }
            }
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<GossipsubEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, message: GossipsubEvent) {
        if let GossipsubEvent::Message(_, message) = message {
            self.events.push(MyBehaviourEvent::GossipMessage {
                source: message.source,
                topics: message.topics,
                message: message.data,
            })
        }
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> MyBehaviour<TSubstream> {
    /// Consumes the events list when polled.
    fn poll<TBehaviourIn>(
        &mut self,
    ) -> Async<NetworkBehaviourAction<TBehaviourIn, MyBehaviourEvent>> {
        if !self.events.is_empty() {
            return Async::Ready(NetworkBehaviourAction::GenerateEvent(self.events.remove(0)));
        }
        Async::NotReady
    }
}

impl<TSubstream: AsyncRead + AsyncWrite> MyBehaviour<TSubstream> {
    pub fn new(local_key: &Keypair) -> Self {
        let local_peer_id = local_key.public().into_peer_id();
        let gossipsub_config = GossipsubConfig::default();
        let kademlia_config = KademliaConfig::default();
        let store = MemoryStore::new(local_peer_id.clone());
        MyBehaviour {
            gossipsub: Gossipsub::new(local_peer_id.clone(), gossipsub_config),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            kad: Kademlia::with_config(local_peer_id.clone(), store, kademlia_config),
            events: vec![],
        }
    }

    pub fn publish(&mut self, topic: &Topic, data: impl Into<Vec<u8>>) {
        self.gossipsub.publish(topic, data);
    }

    pub fn subscribe(&mut self, topic: Topic) -> bool {
        self.gossipsub.subscribe(topic)
    }
}
