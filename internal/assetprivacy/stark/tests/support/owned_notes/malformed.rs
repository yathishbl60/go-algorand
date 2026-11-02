// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, fixture};
use algorand_asset_privacy_stark::compact::Value32;
use p3_air::check_constraints;
use p3_field::PrimeCharacteristicRing;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub fn reject() {
    invalid_shapes();
    invalid_witnesses();
    invalid_cells();
}

fn invalid_shapes() {
    for (count, rows) in [(0, 512), (3, 2048), (2, 512), (1, 513), (1, 131072)] {
        assert!(Air::new([3; 32], 7, [19; 64], [41; 64], count, rows).is_none());
    }
}

fn invalid_witnesses() {
    let air = fixture::air(1, 512);
    let (mut notes, secrets) = fixture::inputs(1, 0);
    assert!(air.trace(&notes, &[[0; 32]]).is_none());
    assert!(air.trace(&notes, &fixture::inputs(1, 17).1).is_none());
    assert!(air.trace(&[], &secrets).is_none());
    assert!(air.trace(&notes, &[]).is_none());
    notes[0][511] = 1;
    assert!(air.trace(&notes, &secrets).is_none());
}

fn invalid_cells() {
    let air = fixture::air(1, 512);
    let (notes, secrets) = fixture::inputs(1, 0);
    let trace = air.trace(&notes, &secrets).expect("valid fixture");
    // Fault injection into the secret inverse and the trusted note-head relay.
    for column in [1964, 1965 + 20] {
        let mut changed = trace.clone();
        changed.values[column] = Value32::ZERO;
        let result = catch_unwind(AssertUnwindSafe(|| {
            check_constraints(&air, &changed, air.public_values());
        }));
        assert!(result.is_err());
    }
}
