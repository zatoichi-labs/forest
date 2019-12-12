use libp2p::core::{
    upgrade::{self, Negotiated},
    InboundUpgrade, OutboundUpgrade, UpgradeInfo,
};
use tokio::prelude::*;

use std::borrow::Cow;
use std::iter;

pub struct ProtocolConfig {
    protocol_id: Cow<'static, [u8]>,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            protocol_id: Cow::Borrowed(b"/ipfs/bitswap"),
        }
    }
}

impl UpgradeInfo for ProtocolConfig {
    type Info = Cow<'static, [u8]>;
    type InfoIter = iter::Once<Self::Info>;

    fn protocol_info(&self) -> Self::InfoIter {
        iter::once(self.protocol_id.clone())
    }
}
