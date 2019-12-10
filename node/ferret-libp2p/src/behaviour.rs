use futures::Async;
use libp2p::core::identity::Keypair;
use libp2p::core::PeerId;
use libp2p::gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, Topic, TopicHash};
use libp2p::mdns::{Mdns, MdnsEvent};
use libp2p::ping::{
    handler::{PingFailure, PingSuccess},
    Ping, PingEvent,
};
use libp2p::swarm::{NetworkBehaviourAction, NetworkBehaviourEventProcess};
use libp2p::tokio_io::{AsyncRead, AsyncWrite};
use libp2p::NetworkBehaviour;
use slog::debug;
use slog::Logger;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent", poll_method = "poll")]
pub struct MyBehaviour<TSubstream: AsyncRead + AsyncWrite> {
    pub gossipsub: Gossipsub<TSubstream>,
    pub mdns: Mdns<TSubstream>,
    pub ping: Ping<TSubstream>,
    #[behaviour(ignore)]
    events: Vec<MyBehaviourEvent>,
    #[behaviour(ignore)]
    log: Logger,
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

impl<TSubstream: AsyncRead + AsyncWrite> NetworkBehaviourEventProcess<PingEvent>
    for MyBehaviour<TSubstream>
{
    fn inject_event(&mut self, event: PingEvent) {
        match event {
            PingEvent {
                peer,
                result: Result::Ok(PingSuccess::Ping { rtt }),
            } => {
                debug!(
                    self.log,
                    "PingSuccess::Ping rtt to {} is {} ms",
                    peer.to_base58(),
                    rtt.as_millis()
                );
            }
            PingEvent {
                peer,
                result: Result::Ok(PingSuccess::Pong),
            } => {
                debug!(self.log, "PingSuccess::Pong from {}", peer.to_base58());
            }
            PingEvent {
                peer,
                result: Result::Err(PingFailure::Timeout),
            } => {
                debug!(self.log, "PingFailure::Timeout {}", peer.to_base58());
            }
            PingEvent {
                peer,
                result: Result::Err(PingFailure::Other { error }),
            } => {
                debug!(
                    self.log,
                    "PingFailure::Other {}: {}",
                    peer.to_base58(),
                    error
                );
            }
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
    pub fn new(local_key: &Keypair, log: Logger) -> Self {
        let local_peer_id = local_key.public().into_peer_id();
        let gossipsub_config = GossipsubConfig::default();
        MyBehaviour {
            gossipsub: Gossipsub::new(local_peer_id, gossipsub_config),
            mdns: Mdns::new().expect("Failed to create mDNS service"),
            ping: Ping::default(),
            log,
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
