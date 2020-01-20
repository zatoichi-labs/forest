// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::{EPostProof, RawBlock, Ticket, TipSetKeys, TxMeta};
use address::Address;
use cid::Cid;
use clock::ChainEpoch;
use crypto::Signature;
use derive_builder::Builder;
use encoding::{
    de::{self, Deserializer},
    ser::{self, Serializer},
    Cbor, Error as EncodingError,
};
use multihash::Hash;
use serde::{Deserialize, Serialize};

/// Header of a block
///
/// Usage:
/// ```
/// use blocks::{BlockHeader, TipSetKeys, Ticket, TxMeta};
/// use address::Address;
/// use cid::{Cid, Codec, Prefix, Version};
/// use clock::ChainEpoch;
/// use crypto::Signature;
///
/// BlockHeader::builder()
///     .miner_address(Address::new_id(0).unwrap()) // optional
///     .bls_aggregate(Signature(vec![])) // optional
///     .parents(TipSetKeys::default()) // optional
///     .weight(0) // optional
///     .epoch(ChainEpoch::default()) // optional
///     .messages(TxMeta::default()) // optional
///     .message_receipts(Cid::default()) // optional
///     .state_root(Cid::default()) // optional
///     .timestamp(0) // optional
///     .ticket(Ticket::default()) // optional
///     .build_and_validate()
///     .unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Builder)]
#[builder(name = "BlockHeaderBuilder")]
pub struct BlockHeader {
    // CHAIN LINKING
    /// Parents is the set of parents this block was based on. Typically one,
    /// but can be several in the case where there were multiple winning ticket-
    /// holders for an epoch
    #[builder(default)]
    parents: TipSetKeys,

    /// weight is the aggregate chain weight of the parent set
    #[builder(default)]
    weight: u64,

    /// epoch is the period in which a new block is generated.
    /// There may be multiple rounds in an epoch
    /// TODO: Investigate this... Epoch is in the spec, but in Lotus, its u64 height
    #[builder(default)]
    epoch: ChainEpoch,

    #[builder(default)]
    height: u64,

    // MINER INFO
    /// miner_address is the address of the miner actor that mined this block
    #[builder(default)]
    miner_address: Address,

    // STATE
    /// messages contains the merkle links for bls_messages and secp_messages
    #[builder(default)]
    messages: Cid,

    /// message_receipts is the Cid of the root of an array of MessageReceipts
    #[builder(default)]
    message_receipts: Cid,

    /// state_root is a cid pointer to the state tree after application of
    /// the transactions state transitions
    #[builder(default)]
    state_root: Cid,

    #[builder(default)]
    fork_signal: u64,

    #[builder(default)]
    signature: Signature,

    #[builder(default)]
    epost_verify: EPostProof,

    // CONSENSUS
    /// timestamp, in seconds since the Unix epoch, at which this block was created
    #[builder(default)]
    timestamp: u64,
    /// the ticket submitted with this block
    #[builder(default)]
    ticket: Ticket,
    // SIGNATURES
    /// aggregate signature of miner in block
    #[builder(default)]
    bls_aggregate: Signature,
    // CACHE
    /// stores the cid for the block after the first call to `cid()`
    /// Does not get serialized
    #[builder(default)]
    cached_cid: Cid,

    /// stores the hashed bytes of the block after the fist call to `cid()`
    /// Does not get serialized
    #[builder(default)]
    cached_bytes: Vec<u8>,
}

// TODO verify format or implement custom serialize/deserialize function (if necessary):
// https://github.com/ChainSafe/ferret/issues/143

impl Cbor for BlockHeader {}

impl RawBlock for BlockHeader {
    /// returns the block raw contents as a byte array
    fn raw_data(&self) -> Result<Vec<u8>, EncodingError> {
        // TODO should serialize block header using CBOR encoding
        self.marshal_cbor()
    }
    /// returns the content identifier of the block
    fn cid(&self) -> Cid {
        self.cid().clone()
    }
    /// returns the hash contained in the block CID
    fn multihash(&self) -> Hash {
        self.cid().prefix().mh_type
    }
}

impl BlockHeader {
    /// Generates a BlockHeader builder as a constructor
    pub fn builder() -> BlockHeaderBuilder {
        BlockHeaderBuilder::default()
    }
    /// Getter for BlockHeader parents
    pub fn parents(&self) -> &TipSetKeys {
        &self.parents
    }
    /// Getter for BlockHeader weight
    pub fn weight(&self) -> u64 {
        self.weight
    }
    /// Getter for BlockHeader epoch
    pub fn epoch(&self) -> &ChainEpoch {
        &self.epoch
    }
    /// Getter for BlockHeader miner_address
    pub fn miner_address(&self) -> &Address {
        &self.miner_address
    }
    /// Getter for BlockHeader messages
    pub fn messages(&self) -> &Cid {
        &self.messages
    }
    /// Getter for BlockHeader message_receipts
    pub fn message_receipts(&self) -> &Cid {
        &self.message_receipts
    }
    /// Getter for BlockHeader state_root
    pub fn state_root(&self) -> &Cid {
        &self.state_root
    }
    /// Getter for BlockHeader timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    /// Getter for BlockHeader ticket
    pub fn ticket(&self) -> &Ticket {
        &self.ticket
    }
    /// Getter for BlockHeader bls_aggregate
    pub fn bls_aggregate(&self) -> &Signature {
        &self.bls_aggregate
    }
    /// Getter for BlockHeader cid
    pub fn cid(&self) -> &Cid {
        // Cache should be initialized, otherwise will return default Cid
        &self.cached_cid
    }
    /// Getter for BlockHeader fork_signal
    pub fn fork_signal(&self) -> u64 {
        self.fork_signal
    }
    /// Getter for BlockHeader epost_verify
    pub fn epost_verify(&self) -> &EPostProof {
        &self.epost_verify
    }
    /// Getter for BlockHeader signature
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
    pub fn height(&self) -> u64 {
        self.height
    }
    /// Updates cache and returns mutable reference of header back
    fn update_cache(&mut self) -> Result<(), String> {
        self.cached_bytes = self.marshal_cbor().map_err(|e| e.to_string())?;
        self.cached_cid = Cid::from_bytes_default(&self.cached_bytes).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl BlockHeaderBuilder {
    pub fn build_and_validate(&self) -> Result<BlockHeader, String> {
        // Convert header builder into header struct
        let mut header = self.build()?;

        // TODO add validation function

        // Fill header cache with raw bytes and cid
        header.update_cache()?;

        Ok(header)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CborBlockHeader(
    Address,    // miner_address
    Ticket,     // ticket
    EPostProof, // epost_verify
    TipSetKeys, // parents []cid
    u64,        // weight
    u64,        // height
//    ChainEpoch, // epoch
    Cid,        // state_root
    Cid,        // message_receipts
    Cid,     // messages
    Signature,  // bls_aggregate
    u64,        // timestamp
    Signature,  // signature
    u64,        // fork_signal
);

impl ser::Serialize for BlockHeader {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let value: CborBlockHeader = CborBlockHeader(
            self.miner_address.clone(),
            self.ticket.clone(),
            self.epost_verify.clone(),
            self.parents.clone(),
            self.weight,
            self.height,
            self.state_root.clone(),
            self.message_receipts.clone(),
            self.messages.clone(),
            self.bls_aggregate.clone(),
            self.timestamp,
            self.signature.clone(),
            self.fork_signal,
        );
        CborBlockHeader::serialize(&value, serializer)
    }
}

impl<'de> de::Deserialize<'de> for BlockHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let cm = CborBlockHeader::deserialize(deserializer)?;
        Ok(Self{
            parents: cm.3,
            weight: cm.4,
            epoch: Default::default(),
            height: cm.5,
            miner_address: cm.0,
            messages: cm.8,
            message_receipts: cm.7,
            state_root: cm.6,
            fork_signal: cm.12,
            signature: cm.11,
            epost_verify: cm.2,
            timestamp: cm.10,
            ticket: cm.1,
            bls_aggregate: cm.9,
            cached_cid: Default::default(),
            cached_bytes: vec![]
        })
    }
}
