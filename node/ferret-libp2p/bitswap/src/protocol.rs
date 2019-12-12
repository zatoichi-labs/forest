use libp2p::core::{InboundUpgrade, OutboundUpgrade, UpgradeInfo, upgrade::{self, Negotiated}};
use tokio::prelude::*;

use std::borrow::Cow;

pub struct ProtocolConfig {
    protocol_id: Cow<'static, [u8]>,
}

impl Default for ProtocolConfig{
    fn default() -> Self{
        Self{
            protocol_id: Cow::Borrowed(b"/ipfs/bitswap")
        }
    }
}

