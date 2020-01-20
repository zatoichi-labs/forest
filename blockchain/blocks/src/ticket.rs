// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crypto::VRFResult;
use encoding::{de::{self, Deserializer} , ser::{self, Serializer}, Cbor};
use serde::{Deserialize, Serialize};

/// A Ticket is a marker of a tick of the blockchain's clock.  It is the source
/// of randomness for proofs of storage and leader election.  It is generated
/// by the miner of a block using a VRF and a VDF.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Default, Serialize, Deserialize)]
pub struct Ticket {
    /// A proof output by running a VRF on the VDFResult of the parent ticket
    pub vrfproof: VRFResult,
}

// TODO verify format or implement custom serialize/deserialize function (if necessary):
// https://github.com/ChainSafe/ferret/issues/143

impl Ticket {
    /// Ticket constructor
    pub fn new(vrfproof: VRFResult) -> Self {
        Self { vrfproof }
    }
}

/// PoSt election candidates
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EPostTicket {
    partial: Vec<u8>,
    sector_id: u64,
    challenge_index: u64,
}

/// Proof of Spacetime election proof
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EPostProof {
    proof: Vec<u8>,
    post_rand: Vec<u8>,
    candidates: Vec<EPostTicket>,
}

#[derive(Serialize, Deserialize)]
pub struct CborEPostTicket (
    Vec<u8>,    // partial
    u64,        // sector_id
    u64         // challenge_index
);

#[derive(Serialize, Deserialize)]
pub struct CborEPostProof (
    Vec<u8>,             // proof
    Vec<u8>,             // post_rand
    Vec<EPostTicket>     // candidates
);

impl ser::Serialize for EPostTicket{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let value: CborEPostTicket = CborEPostTicket(
            self.partial.clone(),
            self.sector_id,
            self.challenge_index
        );
        CborEPostTicket::serialize(&value, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostTicket {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let cm = CborEPostTicket::deserialize(deserializer)?;
        Ok(Self{
            partial: cm.0,
            sector_id: cm.1,
            challenge_index: cm.2
        })
    }
}

impl ser::Serialize for EPostProof {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let value: CborEPostProof = CborEPostProof(
            self.proof.clone(),
            self.post_rand.clone(),
            self.candidates.clone()
        );
        CborEPostProof::serialize(&value, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let cm = CborEPostProof::deserialize(deserializer)?;
        Ok(Self{
            proof: cm.0,
            post_rand: cm.1,
            candidates: cm.2
        })
    }
}