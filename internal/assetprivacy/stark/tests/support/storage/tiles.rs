// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, Matrix, V};
use algorand_asset_privacy_stark::compact_storage::MaskedTrace;
use p3_dft::{Radix2Dit, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::Matrix as MatrixTrait;
use std::ops::Range;

pub fn compare(storage: &MaskedTrace<V>, log_n: usize, dft: &Radix2Dit<V>) {
    let domain = Domain::new(V::from_u8(3), log_n + 4).expect("target coset");
    let mut coefficients = storage.coefficients().clone();
    coefficients
        .values
        .resize(domain.size() * storage.width(), V::ZERO);
    let dense = dft.coset_dft_batch(coefficients, domain.shift());
    for range in [0..1, 1..9, 9..17, 17..27] {
        compare_tile(storage, &dense, range, domain, dft);
    }
}

fn compare_tile(
    storage: &MaskedTrace<V>,
    dense: &Matrix,
    range: Range<usize>,
    domain: Domain<V>,
    dft: &Radix2Dit<V>,
) {
    let limit = domain.size() * range.len();
    let rejected = storage.evaluate_columns(range.clone(), domain, limit - 1, dft);
    assert!(rejected.is_none());
    let tile = storage
        .evaluate_columns(range.clone(), domain, limit, dft)
        .expect("bounded tile");
    compare_rows(&tile, dense, range);
}

fn compare_rows(tile: &Matrix, dense: &Matrix, range: Range<usize>) {
    for row in 0..tile.height() {
        assert_eq!(
            tile.row_slice(row).expect("tile row").as_ref(),
            &dense.row_slice(row).expect("dense row")[range.clone()]
        );
    }
}

pub fn recover(storage: &MaskedTrace<V>, original: &Matrix, domain: Domain<V>, dft: &Radix2Dit<V>) {
    let double = Domain::new(domain.shift(), domain.log_size() + 1).expect("masked domain");
    let masked = storage
        .evaluate_columns(0..17, double, 2 * original.values.len(), dft)
        .expect("recover original trace rows");
    for row in 0..original.height() {
        assert_eq!(
            masked.row_slice(row * 2).expect("masked row").as_ref(),
            original.row_slice(row).expect("original row").as_ref()
        );
    }
    reject_invalid_tiles(storage, domain, double, dft);
}

fn reject_invalid_tiles(
    storage: &MaskedTrace<V>,
    domain: Domain<V>,
    double: Domain<V>,
    dft: &Radix2Dit<V>,
) {
    for (range, target) in [(0..0, double), (0..28, double), (0..1, domain)] {
        let rejected = storage.evaluate_columns(range, target, usize::MAX, dft);
        assert!(rejected.is_none());
    }
}
