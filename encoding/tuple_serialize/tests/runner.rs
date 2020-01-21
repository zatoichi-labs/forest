// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");
    t.pass("tests/overridden_order.rs");
}