
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade::{self, Negotiated, ReadOneError}};
use std::{io, iter};
use tokio::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct HelloConfig {}


impl UpgradeInfo for HelloConfig {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/fil/hello/1.0.0")
    }
}

impl<C> InboundUpgrade<C> for HelloConfig
    where C: AsyncRead + AsyncWrite
{
    type Output = Message<u8>;
    type Error = ReadOneError;
    type Future = upgrade::ReadOneThen<Negotiated<C>, (), fn(Vec<u8>, ()) -> Result<Self::Output, Self::Error>>;

    #[inline]
    fn upgrade_inbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        println!("upgrade_inbound: {}", std::str::from_utf8(info).unwrap());
        upgrade::read_one_then(socket, 50000000, (), |packet, ()| {
            Ok(Message{item: packet})
        })
    }
}
#[derive(Debug)]
pub struct Message <u8>{
    pub item: Vec<u8>
}
impl UpgradeInfo for Message<u8> {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/fil/hello/1.0.0")
    }
}

impl<C> OutboundUpgrade<C> for Message<u8>
    where C: AsyncRead + AsyncWrite,
{
    type Output = ();
    type Error = io::Error;
    type Future = upgrade::WriteOne<Negotiated<C>>;

    #[inline]
    fn upgrade_outbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        println!("upgrade_outbound: {}", std::str::from_utf8(info).unwrap());
        let bytes = self.item;

        upgrade::write_one(socket, bytes)
    }
}

