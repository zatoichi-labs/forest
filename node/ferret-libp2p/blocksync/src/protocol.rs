use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade::{self, Negotiated, ReadOneError}, };
use serde_derive::{Deserialize, Serialize};
use std::{io, iter};
use tokio::prelude::*;
use std::marker::PhantomData;
use cid::Cid;
use std::convert::Infallible;

use tokio::codec::{Framed, Decoder};
use tokio::codec::BytesCodec;
use tokio::prelude::future::AndThen;

//#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct Message{
//    #[serde(with = "CidDef")]
    pub message: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Response{
    pub response: Vec<u8>,
}


//#[derive(Serialize, Deserialize)]
//#[serde(remote = "Cid")]
//pub struct CidDef{
//    pub version: cid::Version,
//    pub codec: cid::Codec,
//    pub hash: Vec<u8>,
//}

#[derive(Clone, Debug, Default)]
pub struct BlockSyncConfig{}

impl UpgradeInfo for BlockSyncConfig {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/fil/sync/blk/0.0.1")
    }
}

impl<C> InboundUpgrade<C> for BlockSyncConfig
    where C: AsyncRead + AsyncWrite
{
    type Output = Message;
    type Error = ReadOneError;
//    type Future = AndThen<Framed<Negotiated<C>, BytesCodec>, fn(BytesMut)-> Message, Message>;
    type Future = upgrade::ReadOneThen<Negotiated<C>, (), fn(Vec<u8>, ()) -> Result<Self::Output, Self::Error>>;

    fn upgrade_inbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        println!("upgrade_inbound: {}", std::str::from_utf8(&info).unwrap());
        upgrade::read_one_then(socket, 524288, (), |packet, ()| {
            let message: Message = Message{
                message: vec![]
            };
            println!("inbound message: {:?}", packet);
            Ok(message)
        })
//        let codec = BytesCodec::new();

//        codec.framed(socket).and_then(|response| {
//            Message{message: response.to_vec()}
//        })
    }
}

impl UpgradeInfo for Message {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/fil/sync/blk/0.0.1")
    }
}

impl<C> OutboundUpgrade<C> for Message
    where
        C: AsyncRead + AsyncWrite,
{
    type Output = ();
//    type Error = Infallible;
    type Error = io::Error;
//    type Future = future::FutureResult<Self::Output, Self::Error>;
    type Future = upgrade::WriteOne<Negotiated<C>>;

    #[inline]
    fn upgrade_outbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        let x: PhantomData<C> = PhantomData;
        println!("upgrade_outbound: {}", std::str::from_utf8(info).unwrap());
//        let bytes = self.write_to_bytes().unwrap();
        let codec = BytesCodec::new();
        let bytes : Vec<u8> = vec![0x83, 0x81, 0xd8, 0x2a, 0x58, 0x27, 0x0, 0x1, 0x71, 0xa0, 0xe4, 0x2, 0x20, 0x2f, 0x7a, 0x58, 0xe0, 0x28, 0x41, 0x6a, 0x7f, 0x69, 0xe0, 0x70, 0x77, 0xfc, 0xfd, 0x5c, 0x42, 0xb3, 0xc, 0xee, 0x48, 0x34, 0x44, 0x2d, 0x2f, 0xb1, 0x28, 0x97, 0x27, 0xab, 0xb1, 0x7a, 0x21, 0x3, 0x0];
        println!("upgrade_outbound in bytes: {:?}", bytes);
//        future::ok(codec.framed(socket))
//       upgrade::write_one(socket, bytes)

//        socket.write(bytes.as_bytes());
    }
}
