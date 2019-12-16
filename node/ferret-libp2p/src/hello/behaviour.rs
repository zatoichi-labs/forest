use libp2p::core::ConnectedPoint;
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::swarm::protocols_handler::{OneShotHandler, ProtocolsHandler};
use libp2p::{Multiaddr, PeerId};
use std::marker::PhantomData;
use super::protocol::{HelloConfig, Message, };
use tokio::prelude::{Async, AsyncWrite, AsyncRead, };
use std::collections::VecDeque;

pub struct Hello<TSubstream>{
    marker: PhantomData<TSubstream>,
    events: VecDeque<NetworkBehaviourAction<Message<u8>, ()>>,
}
#[derive(Debug)]
pub enum HelloEvent{
    ReceiveHello (Message<u8>),
    SentHello,
}

impl From<Message<u8>> for HelloEvent {
    #[inline]
    fn from(message: Message<u8>) -> HelloEvent {
        HelloEvent::ReceiveHello(message)
    }
}

impl From<()> for HelloEvent {
    #[inline]
    fn from(_: ()) -> HelloEvent {
        HelloEvent::SentHello
    }
}

impl<TSubstream> Hello<TSubstream> {
    /// Creates a new `Identify` network behaviour.
    pub fn new() -> Self {
        Hello {
            marker: PhantomData,
            events: VecDeque::new(),
        }
    }
}

impl <TSubstream> NetworkBehaviour for Hello<TSubstream>
    where TSubstream: AsyncRead + AsyncWrite
{
    type ProtocolsHandler = OneShotHandler<TSubstream, HelloConfig, Message<u8>, HelloEvent>;
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
       Default::default()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: PeerId, endpoint: ConnectedPoint) {
        println!("Inject connected");
        println!("peer_id: {}", peer_id.to_base58());
        let message = Message{
            item: vec![1,2,3,4,6]
        };
        self.events.push_back(NetworkBehaviourAction::SendEvent {
            peer_id,
            event: message,
        })
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId, endpoint: ConnectedPoint) {
        println!("Inject Disconnected")
    }

    fn inject_node_event(&mut self, peer_id: PeerId, event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent) {
        match event{
            HelloEvent::ReceiveHello(a) => {
                println!("Received Hello {:?}",a );
            }
            HelloEvent::SentHello => {
                println!("Sent Hello")
            }
        }
    }

    fn poll(&mut self, params: &mut impl PollParameters) -> Async <
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Some(event) = self.events.pop_front() {
            println!("hit");
            return Async::Ready(event);
        }
        Async::NotReady
    }
}
