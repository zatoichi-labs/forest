use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade::{self, Negotiated}};
use std::{io, iter};
use tokio::prelude::*;


#[derive(Clone, Debug, Default)]
pub struct BlockSyncConfig{}

impl UpgradeInfo for BlockSyncConfig {
    type Info = &'static [u8];
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(b"/fil/blocksync/0.0.1")
    }
}

impl<C> InboundUpgrade<C> for BlockSyncConfig
    where C: AsyncRead + AsyncWrite
{
    type Output = ();
    type Error = ();
    type Future = ();

    fn upgrade_inbound(self, socket: Negotiated<C>, info: Self::Info) -> Self::Future {
        unimplemented!()
    }
}

