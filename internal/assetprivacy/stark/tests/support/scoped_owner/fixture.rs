// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, SECRET};
use algorand_asset_privacy_stark::{
    owner_registration_air::PublicStatement,
    verification::selection::{Domain, Pin, Prepared, Registry, encode_registry, registry_digest},
    verification::{DecodeLimits, implementation_id},
};
use sha3::{Digest, Sha3_512};

pub fn new() -> Fixture {
    let prepared = Prepared::owner_registration(domain()).expect("typed owner relation");
    let setup = prepared
        .setup_public()
        .expect("actual public preprocessing");
    let encoded = encode_registry([31; 64], &[setup.entry()]).expect("real scoped registry");
    let pin = pin(&encoded);
    let registry = Registry::load(&encoded, pin).expect("independently pinned scoped registry");
    Fixture {
        prepared,
        setup,
        encoded,
        pin,
        registry,
        statement: statement(),
    }
}

pub fn domain() -> Domain {
    Domain {
        network: [3; 32],
        asset: 7,
        policy: [19; 64],
    }
}

pub fn pin(encoded: &[u8]) -> Pin {
    Pin {
        profile: [31; 64],
        implementation: implementation_id(),
        digest: registry_digest(encoded).expect("fixed registry size"),
    }
}

pub fn limits() -> DecodeLimits {
    DecodeLimits::new(4_900_000, 64 << 20).expect("bounded decoder limits")
}

fn statement() -> PublicStatement {
    let owner = Sha3_512::new()
        .chain_update(b"Algorand/AssetPrivacy/Candidate1/SpendingKey/")
        .chain_update(domain().network)
        .chain_update(SECRET)
        .finalize()
        .into();
    PublicStatement {
        owner,
        group: [41; 64],
    }
}
