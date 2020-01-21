// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use tuple_serialize::TupleSerialize;
use serde;

#[derive(TupleSerialize)]
pub struct Serialize {
    field1: String,
    #[tuple_order(1)]
    field2: u8,
    #[tuple_order(2)]
    field3: (u8, String),
}

fn main() {}
