// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::compact::RANDOM_CODEWORDS;
use p3_field::{TwoAdicField, coset::TwoAdicMultiplicativeCoset};
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use rand::{
    CryptoRng, RngExt,
    distr::{Distribution, StandardUniform},
};

pub(super) fn valid_trace<F: TwoAdicField>(
    trace: &RowMajorMatrix<F>,
    domain: TwoAdicMultiplicativeCoset<F>,
) -> bool {
    let width = trace.width();
    if width == 0 || !trace.values.len().is_multiple_of(width) {
        return false;
    }
    let rows = trace.height();
    (512..=65536).contains(&rows)
        && rows.is_power_of_two()
        && domain.shift() != F::ZERO
        && domain.log_size() == rows.ilog2() as usize
        && domain.log_size() < F::TWO_ADICITY
        && F::two_adic_generator(domain.log_size() + 1).square() == domain.subgroup_generator()
}

pub(super) fn mask<F: TwoAdicField, R: CryptoRng>(
    trace: RowMajorMatrix<F>,
    rng: &mut R,
) -> Option<RowMajorMatrix<F>>
where
    StandardUniform: Distribution<F>,
{
    let old_width = trace.width();
    let width = old_width.checked_add(RANDOM_CODEWORDS)?;
    let paired_width = width.checked_mul(2)?;
    let length = paired_width.checked_mul(trace.height())?;
    let mut values = Vec::new();
    values.try_reserve_exact(length).ok()?;
    values.resize(length, F::ZERO);
    fill(&trace, &mut values, paired_width, rng);
    Some(RowMajorMatrix::new(values, width))
}

fn fill<F: TwoAdicField, R: CryptoRng>(
    trace: &RowMajorMatrix<F>,
    values: &mut [F],
    paired_width: usize,
    rng: &mut R,
) where
    StandardUniform: Distribution<F>,
{
    let old_width = trace.width();
    for (row, original) in values
        .chunks_exact_mut(paired_width)
        .zip(trace.values.chunks_exact(old_width))
    {
        row[..old_width].copy_from_slice(original);
        row[old_width..]
            .iter_mut()
            .for_each(|cell| *cell = rng.random());
    }
}
