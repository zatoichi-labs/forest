use libp2p::core::{
    upgrade::{self, Negotiated, ReadOneError},
    InboundUpgrade, OutboundUpgrade, UpgradeInfo,
};
use super::bitswap::{Message, Message_Wantlist, Message_Block, Message_Wantlist_Entry};
use tokio::prelude::*;

use std::borrow::Cow;
use std::iter;
use std::io;
use protobuf;
use protobuf::Message as ProtobufMessage;
use std::marker::PhantomData;

#[derive(Clone, Debug, Default)]
pub struct BitSwapConfig {}

impl UpgradeInfo for BitSwapConfig {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        // b"/ipfs/bitswap", b"/ipfs/bitswap/1.0.0"
        iter::once(b"/ipfs/bitswap/1.1.0")
    }
}


impl<C> InboundUpgrade<C> for BitSwapConfig
    where C: AsyncRead + AsyncWrite
{
    type Output = Message;
    type Error = ReadOneError;
    type Future = upgrade::ReadOneThen<Negotiated<C>, (), fn(Vec<u8>, ()) -> Result<Self::Output, Self::Error>>;

    #[inline]
    fn upgrade_inbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        println!("upgrade_inbound: {}", std::str::from_utf8(&info).unwrap());
        upgrade::read_one_then(socket, 524288, (), |packet, ()| {
            let message: Message = protobuf::parse_from_bytes(packet.as_ref()).unwrap();
            println!("inbound message: {:?}", message);
            Ok(message)
        })
    }
}

impl UpgradeInfo for Message {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        // b"/ipfs/bitswap", b"/ipfs/bitswap/1.0.0"
        iter::once(b"/ipfs/bitswap/1.1.0")
    }
}

impl<C> OutboundUpgrade<C> for Message
    where
        C: AsyncRead + AsyncWrite,
{
    type Output = ();
    type Error = io::Error;
    type Future = upgrade::WriteOne<Negotiated<C>>;

    #[inline]
    fn upgrade_outbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        let x: PhantomData<C> = PhantomData;
        println!("upgrade_outbound: {}", std::str::from_utf8(info).unwrap());
        let bytes = self.write_to_bytes().unwrap();
        println!("upgrade_outbound in bytes: {:?}", bytes);

        upgrade::write_one(socket, bytes)
    }
}

//fn into_bytes(abc: Message) -> Vec<u8>  {
//    let mut proto = Message::new();
//    let mut wantlist = Message_Wantlist::new();
//
//    let mut entry = Message_Wantlist_Entry::new();
//    entry.set_block(cid.to_bytes());
//    entry.set_priority(1);
//    wantlist.mut_entries().push(entry);
//}
//
