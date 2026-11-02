// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Fixture, OWNER_RECORD, fixture};
use algorand_asset_privacy_stark::verification::selection::{Prepared, Registry, encode_registry};

pub fn reject(fixture: &Fixture) {
    assert_eq!(fixture.encoded.len(), 1774);
    assert_eq!(&fixture.encoded[..5], b"NAPK\x07");
    assert_eq!(fixture.encoded[134 + 17], 0);
    assert_eq!(fixture.encoded[OWNER_RECORD + 17], 3);
    let absent = Prepared::unshield(fixture::domain(), 1).expect("typed absent relation");
    assert!(fixture.registry.bind(absent).is_err());
    assert!(encode_registry([31; 64], &[]).is_err());
    let duplicate = [fixture.setup.entry(), fixture.setup.entry()];
    assert!(encode_registry([31; 64], &duplicate).is_err());
    changed_pins(fixture);
    changed_records(fixture);
    empty_scope(fixture);
    changed_lengths(fixture);
}

fn changed_pins(fixture: &Fixture) {
    for field in 0..3 {
        let mut pin = fixture.pin;
        match field {
            0 => pin.profile[0] ^= 1,
            1 => pin.implementation[0] ^= 1,
            _ => pin.digest[0] ^= 1,
        }
        assert!(Registry::load(&fixture.encoded, pin).is_err());
    }
}

fn changed_records(fixture: &Fixture) {
    // Re-pin malformed files so these checks exercise format semantics, not only hashing.
    for offset in [4, 133, 134, 135, 134 + 17, 134 + 18, OWNER_RECORD + 17] {
        let mut changed = fixture.encoded.clone();
        changed[offset] ^= 1;
        assert!(Registry::load(&changed, fixture::pin(&changed)).is_err());
    }
    let mut false_presence = fixture.encoded.clone();
    false_presence[134 + 17] = 3;
    assert!(Registry::load(&false_presence, fixture::pin(&false_presence)).is_err());
}

fn changed_lengths(fixture: &Fixture) {
    let mut short = fixture.encoded.clone();
    short.pop();
    assert!(Registry::load(&short, fixture.pin).is_err());
    let mut trailing = fixture.encoded.clone();
    trailing.push(0);
    assert!(Registry::load(&trailing, fixture.pin).is_err());
}

fn empty_scope(fixture: &Fixture) {
    let mut absent = fixture.encoded.clone();
    absent[OWNER_RECORD + 17..OWNER_RECORD + 82].fill(0);
    assert!(Registry::load(&absent, fixture::pin(&absent)).is_err());
    let mut zero_root = fixture.encoded.clone();
    zero_root[OWNER_RECORD + 18..OWNER_RECORD + 82].fill(0);
    assert!(Registry::load(&zero_root, fixture::pin(&zero_root)).is_err());
}
