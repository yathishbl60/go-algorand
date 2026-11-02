// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Air, Trace, fixture, pipeline, splicing};
use algorand_asset_privacy_stark::compact::{Challenge32, Config, prover_config};
use p3_air::check_constraints;
use p3_field::PrimeCharacteristicRing;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub fn reject() {
    let config = prover_config().expect("OS entropy");
    let air = fixture::air(2, 1024);
    let (notes, secrets) = fixture::inputs(2, 0);
    let original = air.trace(&notes, &secrets).expect("valid fixture");
    for case in 0..3 {
        let changed = splicing::trace(&air, &original, &notes, case);
        check_constraints(&air, &changed, air.public_values());
        reject_private_link(&config, &air, &changed);
    }
}

fn reject_private_link(config: &Config, air: &Air, changed: &Trace) {
    if cfg!(debug_assertions) {
        // P3 checks global lookup balance before producing a debug proof.
        let result = catch_unwind(AssertUnwindSafe(|| pipeline::make(config, air, changed)));
        assert!(result.is_err());
        return;
    }
    let (proof, data) = pipeline::make(config, air, changed);
    assert_ne!(
        proof.lookup_terminals[0].as_ref().expect("one terminal").0,
        Challenge32::ZERO
    );
    assert!(!pipeline::accepts(config, air, &proof, &data));
}
