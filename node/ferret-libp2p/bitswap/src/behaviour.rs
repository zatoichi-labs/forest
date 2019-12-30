use super::protocol::BitSwapConfig;

use libp2p::core::ConnectedPoint;
use libp2p::swarm::protocols_handler::{DummyProtocolsHandler, OneShotHandler, ProtocolsHandler};
use libp2p::swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters};
use libp2p::{Multiaddr, PeerId};

use super::protocol;
use super::bitswap::{Message, Message_Block, Message_Wantlist, Message_Wantlist_Entry};

use std::marker::PhantomData;
use tokio::prelude::*;
use std::collections::VecDeque;

use cid;
use multihash;

pub struct BitSwap<TSubstream> {
    marker: PhantomData<TSubstream>,
    events: VecDeque<NetworkBehaviourAction<Message, ()>>,
    connected_peers: Vec<PeerId>,
}

impl<TSubstream> BitSwap<TSubstream> {
    pub fn new () -> Self {
        BitSwap{
            marker: PhantomData,
            events: VecDeque::new(),
            connected_peers: Vec::new(),
        }
    }

    pub fn send_want_list(&mut self) {
        println!("bitswap: send_want_list");
        let mut message = Message::new();
        let mut wantlist = Message_Wantlist::new();
        let mut wantlist_entry = Message_Wantlist_Entry::new();
        let mut block_1 = Message_Block::new();
//        let hash = "bafy2bzaceaxxuwhafbawu73j4byhp7h5lrblgdhoja2eiljpweujoj5lwf5cc".as_bytes();
//        println!("DECODED: {:?}", multihash::Multihash::from_bytes(hash.to_vec()));
//        let genesisCid = cid::Cid::new(cid::Codec::DagCBOR, cid::Version::V1, "bafy2bzaceaxxuwhafbawu73j4byhp7h5lrblgdhoja2eiljpweujoj5lwf5cc".as_bytes());
//        let genesisCid = cid::Cid::from(hash).unwrap();
//        println!("GENESIS CID {:?}", genesisCid);
        let genesisCidBytes:[u8; 38]  = [
            1, 113, 160, 228,   2,  32,  47, 122,  88,
            224,  40,  65, 106, 127, 105, 224, 112, 119,
            252, 253,  92,  66, 179,  12, 238,  72,  52,
            68,  45,  47, 177,  40, 151,  39, 171, 177,
            122,  33
        ];
        wantlist_entry.set_block(genesisCidBytes.to_vec());
        wantlist_entry.set_priority(1);
        wantlist.mut_entries().push(wantlist_entry);
        message.set_wantlist(wantlist);
        let peer_id = self.connected_peers.first().unwrap();
        self.events.push_back(NetworkBehaviourAction::SendEvent {
            peer_id: peer_id.clone(),
            event: message
        });

        println!("End send_want_list");


    }
}

#[derive(Debug)]
pub enum BitSwapEvent {
    /// We received a `Message` from a remote.
    Rx(Message),
    /// We successfully sent a `Message`.
    Tx,
}

impl From<Message> for BitSwapEvent {
    #[inline]
    fn from(message: Message) -> BitSwapEvent {
        BitSwapEvent::Rx(message)
    }
}

impl From<()> for BitSwapEvent {
    #[inline]
    fn from(_: ()) -> BitSwapEvent {
        BitSwapEvent::Tx
    }
}

impl<TSubstream> NetworkBehaviour for BitSwap<TSubstream>
where
    TSubstream: AsyncRead + AsyncWrite,
{
    type ProtocolsHandler = OneShotHandler<TSubstream, BitSwapConfig, Message, BitSwapEvent>;
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        Default::default()
    }

    fn addresses_of_peer(&mut self, peer_id: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, peer_id: PeerId, endpoint: ConnectedPoint) {
        println!("Adding {:?}", peer_id);
        self.connected_peers.push(peer_id);
    }

    fn inject_disconnected(&mut self, peer_id: &PeerId, endpoint: ConnectedPoint) {
    }

    fn inject_node_event(
        &mut self,
        peer_id: PeerId,
        event: BitSwapEvent,
    ) {
        println!("received event {:?}", event);

        let message = match event {
            BitSwapEvent::Rx(message) => {
                message
            },
            BitSwapEvent::Tx => {
                return;
            },
        };
    }

    fn poll(
        &mut self,
        _: &mut impl PollParameters,
    ) -> Async<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Some(event) = self.events.pop_front() {
            return Async::Ready(event);
        }
        Async::NotReady
    }
}
