// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Domain, V};
use algorand_asset_privacy_stark::compact_storage::MaskedTrace;
use p3_dft::Radix2Dit;
use p3_field::PrimeCharacteristicRing;
use rand::{SeedableRng, rngs::StdRng};

pub fn traces(widths: &[usize]) -> Vec<MaskedTrace<V>> {
    widths
        .iter()
        .enumerate()
        .map(|(index, &width)| masked(index, width))
        .collect()
}

pub fn domain(log: usize, shift: u8) -> Domain {
    Domain::new(V::from_u8(shift), log).expect("valid evaluation coset")
}

fn masked(index: usize, width: usize) -> MaskedTrace<V> {
    masked_rows(index, width, 512)
}

pub fn taller() -> MaskedTrace<V> {
    masked_rows(0, 1, 1024)
}

fn masked_rows(index: usize, width: usize, rows: usize) -> MaskedTrace<V> {
    let values = (0..rows * width)
        .map(|cell| V::from_usize((index + 1) * 1024 + cell * 7 + 13))
        .collect();
    let trace = Dense::new(values, width);
    let dft = Radix2Dit::default();
    let mut rng = StdRng::seed_from_u64(31 + index as u64);
    let source = domain(rows.ilog2() as usize, 5);
    MaskedTrace::from_trace(trace, source, &dft, &mut rng).expect("fixture coefficients")
}
