// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, Matrix, V, tiles};
use algorand_asset_privacy_stark::compact_storage::MaskedTrace;
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use rand::{SeedableRng, rngs::StdRng};

pub fn compare() {
    for log_n in [9, 10, 12] {
        for shift in [V::ONE, V::from_u8(11)] {
            compare_case(log_n, shift);
        }
    }
}

fn compare_case(log_n: usize, shift: V) {
    let rows = 1usize << log_n;
    let original = trace(rows);
    let domain = Domain::new(shift, log_n).expect("trace domain");
    let dft = Radix2Dit::default();
    // Public deterministic fixture RNG, never a production prover seed.
    let mut rng = StdRng::seed_from_u64(91);
    let storage = MaskedTrace::from_trace(original.clone(), domain, &dft, &mut rng)
        .expect("valid masked trace");
    assert_eq!((storage.trace_rows(), storage.relation_width()), (rows, 17));
    assert_eq!(
        (storage.coefficient_rows(), storage.width()),
        (rows * 2, 27)
    );
    let expected = stock_coefficients(&original, shift);
    assert_eq!(storage.coefficients(), &expected);
    tiles::compare(&storage, log_n, &dft);
    tiles::recover(&storage, &original, domain, &dft);
}

fn trace(rows: usize) -> Matrix {
    Matrix::new(
        (0..rows * 17).map(|i| V::from_usize(i * 7 + 13)).collect(),
        17,
    )
}

fn stock_coefficients(original: &Matrix, shift: V) -> Matrix {
    let mut masked = original
        .clone()
        .with_random_cols(37, StdRng::seed_from_u64(91));
    masked.width = 27;
    Radix2Dit::default().coset_idft_batch(masked, shift)
}
