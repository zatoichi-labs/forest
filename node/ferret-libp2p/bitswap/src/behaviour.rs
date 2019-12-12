use super::protocol::ProtocolConfig;

use libp2p::core::ConnectedPoint;
use libp2p::swarm::protocols_handler::{DummyProtocolsHandler, OneShotHandler, ProtocolsHandler};
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::{Multiaddr, PeerId};

use std::marker::PhantomData;
use tokio::prelude::*;

pub struct BitSwap<TSubstream> {
    marker: PhantomData<TSubstream>,
}

impl<TSubstream> BitSwap<TSubstream> {}

impl<TSubstream> NetworkBehaviour for BitSwap<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type ProtocolsHandler = DummyProtocolsHandler<TSubstream>;
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        unimplemented!()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        unimplemented!()
    }

    fn inject_connected(&mut self, peer_id: PeerId, endpoint: ConnectedPoint) {
        unimplemented!()
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId, endpoint: ConnectedPoint) {
        unimplemented!()
    }

    fn inject_node_event(
        &mut self,
        peer_id: PeerId,
        event: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
        unimplemented!()
    }

    fn poll(
        &mut self,
        _: &mut impl PollParameters,
    ) -> Async<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
            Self::OutEvent,
        >,
    > {
        unimplemented!()
    }
}
