// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, SALT_SEED, V, fixture};
use algorand_asset_privacy_stark::compact_storage::{MaskedTrace, StreamedTree};
use p3_dft::Radix2Dit;
use rand::{SeedableRng, rngs::StdRng};

pub fn reject() {
    let inputs = fixture::traces(&[1]);
    let references: Vec<_> = inputs.iter().collect();
    let domain = fixture::domain(10, 3);
    assert!(!accepts(&[], domain, 1, 1024));
    assert!(!accepts(&references, domain, 0, 1024));
    assert!(!accepts(&references, domain, 1, 1023));
    assert!(!accepts(&references, fixture::domain(9, 3), 1, usize::MAX));
    let excessive = vec![&inputs[0]; 17];
    assert!(!accepts(&excessive, domain, 1, 1024));
    let taller = fixture::taller();
    assert!(!accepts(
        &[&inputs[0], &taller],
        fixture::domain(11, 3),
        1,
        2048
    ));
}

fn accepts(inputs: &[&MaskedTrace<V>], domain: Domain, tile: usize, budget: usize) -> bool {
    StreamedTree::commit(
        inputs,
        domain,
        tile,
        budget,
        &Radix2Dit::default(),
        &mut StdRng::seed_from_u64(SALT_SEED),
    )
    .is_some()
}
