// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Domain, Fixture, InputMmcs, MaskedTrace, ROWS, V, configuration};
use algorand_asset_privacy_stark::compact_storage::StreamedInputLimits;
use p3_commit::Mmcs;
use p3_dft::Radix2Dit;
use p3_field::{Field, PrimeCharacteristicRing};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use std::sync::Arc;

pub fn new() -> Fixture {
    let source = source();
    let evaluation = Domain::new(V::GENERATOR, 13).unwrap();
    let input = InputMmcs::new().expect("input MMCS entropy");
    let (cap, data) = input
        .commit_coefficients(vec![source.clone()], evaluation, limits())
        .unwrap();
    dimensions(&input, &data);
    Fixture {
        source,
        input,
        data,
        cap,
        params: configuration::params(),
        evaluation,
    }
}

fn source() -> Arc<MaskedTrace<V>> {
    let raw = RowMajorMatrix::new((0..512).map(|i| V::from_usize(i + 1)).collect(), 1);
    let mask = MaskedTrace::from_trace(
        raw,
        Domain::new(V::ONE, 9).unwrap(),
        &Radix2Dit::default(),
        &mut configuration::entropy(),
    )
    .expect("stock masked source");
    Arc::new(mask)
}

fn limits() -> StreamedInputLimits {
    StreamedInputLimits {
        tile_columns: 4,
        max_tile_values: ROWS * 4,
        max_output_values: 192 * 11,
    }
}

fn dimensions(input: &InputMmcs, data: &super::CoefficientInputData) {
    let matrices = input.get_matrices(data);
    assert_eq!(matrices.len(), 1);
    assert_eq!((matrices[0].height(), matrices[0].width()), (ROWS, 11));
}
