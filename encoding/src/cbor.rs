// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::errors::Error;
use crate::{de, from_slice, ser, to_vec};

/// Implemented for types that are CBOR encodable
pub trait Cbor: ser::Serialize {
    /// Serializes the object as cbor
    fn marshal_cbor(&self) -> Result<Vec<u8>, Error> {
        Ok(to_vec(&self)?)
    }

    /// Unmarshals cbor encoded bytes to object
    fn unmarshal_cbor_owned(bz: &[u8]) -> Result<Self, Error>
    where
        Self: de::DeserializeOwned,
    {
        Ok(from_slice(bz)?)
    }

    /// Unmarshals cbor encoded bytes to object
    fn unmarshal_cbor<'de>(bz: &'de [u8]) -> Result<Self, Error>
    where
        Self: de::Deserialize<'de>,
    {
        Ok(from_slice::<'de>(bz)?)
    }
}
