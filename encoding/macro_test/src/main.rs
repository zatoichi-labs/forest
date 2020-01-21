// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde;
use tuple_serialize::TupleSerialize;

#[derive(TupleSerialize)]
pub struct MyStruct {
    field1: String,
    field2: u8,
    field3: (u8, String),
}

// impl serde::ser::Serialize for MyStruct {
//     fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::ser::Serializer,
//     {
//         (&self.executable, &self.integer, &self.tuple).serialize(s)
//     }
// }

// impl<'de> serde::de::Deserialize<'de> for MyStruct {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::de::Deserializer<'de>,
//     {
//         let (executable, integer, tuple) = serde::Deserialize::deserialize(deserializer)?;
//         Ok(Self {
//             executable,
//             integer,
//             tuple,
//         })
//     }
// }

fn main() {}
