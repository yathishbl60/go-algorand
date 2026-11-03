// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Domain, Mask, V};
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, bitrev::BitReversibleMatrix};
use rand::{SeedableRng, rngs::StdRng};

pub fn domain(log: usize, shift: u8) -> Domain {
    Domain::new(V::from_u8(shift), log).expect("valid public evaluation coset")
}

pub fn masked(rows: usize, width: usize) -> Mask {
    let values = (0..rows * width)
        .map(|i| V::from_usize(i * 7 + 13))
        .collect();
    let trace = Dense::new(values, width);
    let source = domain(rows.ilog2() as usize, 5);
    // Deliberately public deterministic fixture randomness, never production masks.
    let mut rng = StdRng::seed_from_u64(91);
    Mask::from_trace(trace, source, &Radix2Dit::default(), &mut rng).expect("fixture coefficients")
}

pub fn dense(mask: &Mask, domain: Domain) -> Dense {
    let mut coefficients = mask.coefficients().clone();
    coefficients
        .values
        .resize(domain.size() * mask.width(), V::ZERO);
    let natural = Radix2Dit::default().coset_dft_batch(coefficients, domain.shift());
    natural.bit_reverse_rows().to_row_major_matrix()
}
