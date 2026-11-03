// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{E, Fixture, LOG_ROWS, ROWS, V};
use p3_dft::Radix2Dit;
use p3_field::{PrimeCharacteristicRing, batch_multiplicative_inverse, dot_product};
use p3_matrix::dense::RowMajorMatrix;

pub fn openings(fixture: &Fixture, point: E) -> Vec<E> {
    let mut values = vec![E::ZERO; fixture.source.width()];
    for row in fixture
        .source
        .coefficients()
        .values
        .chunks_exact(values.len())
        .rev()
    {
        for (value, &coefficient) in values.iter_mut().zip(row) {
            *value = *value * point + E::from(coefficient);
        }
    }
    values
}

pub fn word(fixture: &Fixture, point: E, alpha: E, values: &[E]) -> Vec<E> {
    let width = fixture.source.width();
    let dense = dense(fixture);
    let denominators: Vec<_> = (0..ROWS)
        .map(|row| point - E::from(fixture.evaluation.element(natural(row))))
        .collect();
    let inverses = batch_multiplicative_inverse(&denominators);
    let claim: E = dot_product(alpha.powers(), values.iter().copied());
    (0..ROWS)
        .map(|row| {
            let start = natural(row) * width;
            let compressed: E = dot_product(
                alpha.powers(),
                dense.values[start..start + width].iter().copied(),
            );
            (claim - compressed) * inverses[row]
        })
        .collect()
}

fn dense(fixture: &Fixture) -> RowMajorMatrix<V> {
    let width = fixture.source.width();
    fixture
        .source
        .evaluate_columns(
            0..width,
            fixture.evaluation,
            ROWS * width,
            &Radix2Dit::default(),
        )
        .expect("small test-only dense reduction")
}

fn natural(row: usize) -> usize {
    row.reverse_bits() >> (usize::BITS as usize - LOG_ROWS)
}
