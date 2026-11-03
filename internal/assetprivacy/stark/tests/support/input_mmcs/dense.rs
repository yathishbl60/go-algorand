// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Dense, Proof, TreeCap, V, Values, fixture};
use algorand_asset_privacy_stark::compact_storage::{InputMmcs, InputProverData};
use p3_commit::{BatchOpeningRef, Mmcs};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Dimensions, Matrix};

pub fn compare() {
    let input = InputMmcs::new().expect("OS entropy");
    let matrices = vec![matrix(64, 3), matrix(32, 1)];
    let dimensions: Vec<_> = matrices.iter().map(Matrix::dimensions).collect();
    let (cap, data) = input.commit(matrices.clone());
    let indices = [63, 0, 2, 1, 63];
    let (values, proof) = input
        .try_open_multi(&indices, &data)
        .expect("stock mixed-height opening");
    assert!(
        input
            .verify_multi_batch(&cap, &dimensions, &indices, &values, &proof)
            .is_ok()
    );
    verify_stock(&cap, &dimensions, &indices, &values, &proof);
    check_rows(&indices, &values, &matrices);
    single(&input, &cap, &dimensions, &data);
}

fn verify_stock(
    cap: &TreeCap,
    dimensions: &[Dimensions],
    indices: &[usize],
    values: &Values,
    proof: &Proof,
) {
    assert!(
        fixture::oracle()
            .verify_multi_batch(cap, dimensions, indices, values, proof)
            .is_ok()
    );
}

fn single(
    input: &InputMmcs,
    cap: &TreeCap,
    dimensions: &[Dimensions],
    data: &InputProverData<Dense>,
) {
    assert!(input.try_open_batch(64, data).is_err());
    assert!(input.try_open_multi(&[64], data).is_err());
    let opening = input.try_open_batch(17, data).expect("dense single");
    let borrowed = BatchOpeningRef::new(&opening.opened_values, &opening.opening_proof);
    assert!(
        fixture::oracle()
            .verify_batch(cap, dimensions, 17, borrowed)
            .is_ok()
    );
}

fn matrix(rows: usize, width: usize) -> Dense {
    Dense::new(
        (0..rows * width)
            .map(|i| V::from_usize(i * 7 + 13))
            .collect(),
        width,
    )
}

fn check_rows(indices: &[usize], values: &[Vec<Vec<V>>], matrices: &[Dense]) {
    for (&index, rows) in indices.iter().zip(values) {
        for (matrix, actual) in matrices.iter().zip(rows) {
            let row = if matrix.height() == 64 {
                index
            } else {
                index >> 1
            };
            assert_eq!(
                actual,
                &matrix.values[row * matrix.width..(row + 1) * matrix.width]
            );
        }
    }
}
