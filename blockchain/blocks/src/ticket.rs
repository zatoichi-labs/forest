// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crypto::VRFResult;
use encoding::{
    de::{self, Deserializer},
    ser::{self, Serializer},
    serde_bytes,
};
use serde::{Deserialize, Serialize};

/// A Ticket is a marker of a tick of the blockchain's clock.  It is the source
/// of randomness for proofs of storage and leader election.  It is generated
/// by the miner of a block using a VRF and a VDF.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Default)]
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

impl ser::Serialize for Ticket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = [self.vrfproof.clone()];
        value.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Ticket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [cm]: [VRFResult; 1] = Deserialize::deserialize(deserializer)?;
        Ok(Self { vrfproof: cm })
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
pub struct CborEPostTicket(
    #[serde(with = "serde_bytes")] Vec<u8>, // partial
    u64,                                    // sector_id
    u64,                                    // challenge_index
);

#[derive(Serialize, Deserialize)]
pub struct CborEPostProof(
    #[serde(with = "serde_bytes")] Vec<u8>, // proof
    #[serde(with = "serde_bytes")] Vec<u8>, // post_rand
    Vec<EPostTicket>,                       // candidates
);

impl ser::Serialize for EPostTicket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value: CborEPostTicket =
            CborEPostTicket(self.partial.clone(), self.sector_id, self.challenge_index);
        CborEPostTicket::serialize(&value, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostTicket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cm = CborEPostTicket::deserialize(deserializer)?;
        Ok(Self {
            partial: cm.0,
            sector_id: cm.1,
            challenge_index: cm.2,
        })
    }
}

impl ser::Serialize for EPostProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value: CborEPostProof = CborEPostProof(
            self.proof.clone(),
            self.post_rand.clone(),
            self.candidates.clone(),
        );
        CborEPostProof::serialize(&value, serializer)
    }
}

impl<'de> de::Deserialize<'de> for EPostProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cm = CborEPostProof::deserialize(deserializer)?;
        Ok(Self {
            proof: cm.0,
            post_rand: cm.1,
            candidates: cm.2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64;
    use encoding::{from_slice, to_vec};
    use hex::encode;

    // From Lotus
    const TICKET: [u8; 99] = [
        0x81, 0x58, 0x60, 0x96, 0x64, 0x49, 0x2f, 0x30, 0xe9, 0xb9, 0x50, 0x3b, 0x71, 0x41, 0x0b,
        0x1d, 0x38, 0x2e, 0x2b, 0xd4, 0x85, 0x7f, 0xe2, 0x15, 0x39, 0xac, 0x92, 0x1b, 0xcb, 0x7f,
        0xd0, 0x86, 0xd5, 0x78, 0x71, 0xe6, 0xdd, 0x5c, 0x31, 0xcd, 0x23, 0x61, 0x8b, 0x52, 0x52,
        0xb6, 0x2c, 0x7b, 0x44, 0x4c, 0x3a, 0x02, 0x9b, 0xba, 0xad, 0xc2, 0x50, 0x57, 0x56, 0x81,
        0x06, 0x47, 0x77, 0xf6, 0x04, 0x06, 0xc4, 0xff, 0x00, 0x6f, 0x38, 0xfc, 0x61, 0x71, 0xfe,
        0x45, 0xd4, 0x83, 0xe5, 0x15, 0x79, 0xd0, 0xe2, 0x47, 0x8b, 0x7e, 0x5f, 0xde, 0x2c, 0x51,
        0xd2, 0xe8, 0x64, 0x63, 0xaf, 0x86, 0xd3, 0xcb, 0xd5,
    ];
    const EPOST_PROOF: [u8; 333] = [
        0x83, 0x58, 0xc0, 0xae, 0x7f, 0x39, 0xba, 0x2a, 0x1d, 0x0f, 0x6f, 0x71, 0xbe, 0x02, 0x2e,
        0xbc, 0xde, 0x7f, 0x83, 0x7e, 0xc8, 0x5e, 0x08, 0x4f, 0xb5, 0x5b, 0x65, 0xde, 0x58, 0xbd,
        0xcb, 0xe9, 0xcf, 0x1c, 0x2b, 0x9e, 0x01, 0x32, 0x35, 0xab, 0x5f, 0xe8, 0x3a, 0x7d, 0x05,
        0x10, 0x80, 0xd7, 0x45, 0x61, 0xcb, 0xa5, 0x9e, 0x02, 0xcc, 0x0a, 0x8e, 0x75, 0x08, 0x7d,
        0xad, 0xd1, 0xe2, 0x87, 0xe0, 0x48, 0xe4, 0x8b, 0x1d, 0x23, 0x56, 0x29, 0xc1, 0x5f, 0x94,
        0x74, 0xd0, 0xec, 0xa4, 0x95, 0x56, 0xfd, 0xc8, 0xa7, 0x54, 0x4f, 0x99, 0xa2, 0x23, 0xbc,
        0xe9, 0xaa, 0x77, 0xd2, 0x5e, 0xfb, 0x44, 0xb9, 0x2b, 0x13, 0x0c, 0x54, 0x67, 0x9b, 0xfc,
        0x6a, 0x9b, 0x12, 0x45, 0x48, 0xb3, 0xa1, 0x78, 0x75, 0x20, 0x5a, 0xc7, 0x80, 0xad, 0x3a,
        0x82, 0x4d, 0x70, 0x97, 0x92, 0xda, 0xc5, 0x8d, 0xa2, 0xfc, 0x24, 0x20, 0x06, 0x85, 0x88,
        0x3f, 0x1f, 0x68, 0xd8, 0x46, 0x0c, 0x05, 0xb3, 0x5f, 0x41, 0xcb, 0xbe, 0xa5, 0x1c, 0xc5,
        0x9a, 0x20, 0xe3, 0xcd, 0x3e, 0x81, 0x22, 0x16, 0x2b, 0x3d, 0xba, 0x3e, 0x82, 0x6e, 0xb0,
        0x1c, 0x58, 0x8d, 0x86, 0x9d, 0xc5, 0xbc, 0x0b, 0x92, 0x50, 0x7d, 0xbf, 0x37, 0xee, 0x4c,
        0x29, 0x9a, 0x3b, 0x12, 0x1e, 0xcb, 0xc2, 0x01, 0x8b, 0x73, 0x47, 0xcb, 0xe0, 0xc1, 0x08,
        0x58, 0x60, 0x85, 0xda, 0x1d, 0x70, 0x2c, 0xf9, 0x90, 0xb2, 0x58, 0x45, 0xbf, 0x4f, 0x4f,
        0xb9, 0xb8, 0xcf, 0xd9, 0x11, 0xbd, 0xcf, 0x61, 0xd3, 0x62, 0x8c, 0xc9, 0xef, 0x43, 0x3a,
        0x49, 0x67, 0x43, 0xcb, 0xf4, 0xe5, 0x7d, 0x9d, 0xb3, 0xda, 0xe0, 0x36, 0x17, 0x13, 0x57,
        0xe7, 0x7f, 0x71, 0x74, 0xbe, 0x02, 0xf0, 0x03, 0x1e, 0x97, 0xa9, 0x40, 0xe0, 0xcc, 0x57,
        0xfe, 0x84, 0xd6, 0x46, 0xd3, 0xf7, 0xd9, 0x1d, 0x16, 0xdd, 0x31, 0x30, 0xd5, 0x2c, 0x3b,
        0xff, 0x58, 0x6c, 0x7e, 0x2e, 0x8e, 0x27, 0xfb, 0xb1, 0x8d, 0x0f, 0xf2, 0x98, 0x11, 0x02,
        0xe9, 0xe5, 0x32, 0x03, 0xeb, 0xc7, 0xb4, 0xb1, 0x81, 0x83, 0x58, 0x20, 0x4c, 0x59, 0x62,
        0x53, 0xaf, 0xe9, 0x75, 0xb8, 0xd1, 0xca, 0x89, 0x9e, 0x8e, 0x55, 0xcc, 0x4b, 0xbe, 0xea,
        0x8d, 0x87, 0x4c, 0x0e, 0xdc, 0xb4, 0xee, 0xf8, 0xa0, 0xbd, 0x71, 0xff, 0xbe, 0x32, 0x19,
        0x01, 0x1c, 0x05,
    ];

    fn construct_ticket() -> Ticket {
        let vrf_result = VRFResult::new(base64::decode("lmRJLzDpuVA7cUELHTguK9SFf+IVOaySG8t/0IbVeHHm3VwxzSNhi1JStix7REw6Apu6rcJQV1aBBkd39gQGxP8Abzj8YXH+RdSD5RV50OJHi35f3ixR0uhkY6+G08vV").unwrap());
        Ticket::new(vrf_result)
    }

    fn construct_epost_proof() -> EPostProof {
        let etik = EPostTicket {
            partial: base64::decode("TFliU6/pdbjRyomejlXMS77qjYdMDty07vigvXH/vjI=").unwrap(),
            sector_id: 284,
            challenge_index: 5,
        };

        EPostProof{
            proof: base64::decode("rn85uiodD29xvgIuvN5/g37IXghPtVtl3li9y+nPHCueATI1q1/oOn0FEIDXRWHLpZ4CzAqOdQh9rdHih+BI5IsdI1YpwV+UdNDspJVW/cinVE+ZoiO86ap30l77RLkrEwxUZ5v8apsSRUizoXh1IFrHgK06gk1wl5LaxY2i/CQgBoWIPx9o2EYMBbNfQcu+pRzFmiDjzT6BIhYrPbo+gm6wHFiNhp3FvAuSUH2/N+5MKZo7Eh7LwgGLc0fL4MEI").unwrap(),
            post_rand: base64::decode("hdodcCz5kLJYRb9PT7m4z9kRvc9h02KMye9DOklnQ8v05X2ds9rgNhcTV+d/cXS+AvADHpepQODMV/6E1kbT99kdFt0xMNUsO/9YbH4ujif7sY0P8pgRAunlMgPrx7Sx").unwrap(),
            candidates: vec![etik]
        }
    }

    #[test]
    fn encode_ticket() {
        let ticket = construct_ticket();
        // Encode Ticket
        let encoded_ticket = to_vec(&ticket).unwrap();
        assert_eq!(&TICKET[..], &encoded_ticket[..]);
    }

    #[test]
    fn decode_ticket() {
        let ticket = construct_ticket();
        // Decode Ticket
        let decoded_ticket: Ticket = from_slice(&TICKET).unwrap();
        assert_eq!(ticket, decoded_ticket);
    }

    #[test]
    fn encode_epost_proof() {
        let proof = construct_epost_proof();
        // Encode Proof
        let encoded_proof = to_vec(&proof).unwrap();
        assert_eq!(&EPOST_PROOF[..], &encoded_proof[..]);
    }

    #[test]
    fn decode_epost_proof() {
        let proof = construct_epost_proof();
        // Decode Proof
        let decoded_proof: EPostProof = from_slice(&EPOST_PROOF).unwrap();
        assert_eq!(proof, decoded_proof);
    }
}
