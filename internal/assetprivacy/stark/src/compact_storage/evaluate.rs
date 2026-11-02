// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MaskedTrace;
use core::ops::Range;
use p3_dft::TwoAdicSubgroupDft;
use p3_field::{TwoAdicField, coset::TwoAdicMultiplicativeCoset};
use p3_matrix::dense::RowMajorMatrix;

impl<F: TwoAdicField> MaskedTrace<F> {
    /// Evaluate a column tile in natural row order on the requested coset.
    /// `max_values` bounds this tile's scalar buffer, not FFT scratch or total RSS.
    /// The result remains PRIVATE; random columns may be selected explicitly.
    pub fn evaluate_columns<D: TwoAdicSubgroupDft<F, Evaluations = RowMajorMatrix<F>>>(
        &self,
        columns: Range<usize>,
        domain: TwoAdicMultiplicativeCoset<F>,
        max_values: usize,
        dft: &D,
    ) -> Option<RowMajorMatrix<F>> {
        let width = valid_tile(self, &columns, domain)?;
        let rows = 1usize.checked_shl(u32::try_from(domain.log_size()).ok()?)?;
        let length = rows.checked_mul(width)?;
        if length > max_values {
            return None;
        }
        let coefficients = select(self, columns, length)?;
        Some(dft.coset_dft_batch(coefficients, domain.shift()))
    }
}

fn valid_tile<F: TwoAdicField>(
    trace: &MaskedTrace<F>,
    columns: &Range<usize>,
    domain: TwoAdicMultiplicativeCoset<F>,
) -> Option<usize> {
    if columns.start >= columns.end || columns.end > trace.width() || domain.shift() == F::ZERO {
        return None;
    }
    let coefficient_log = trace.coefficient_rows().ilog2() as usize;
    if domain.log_size() < coefficient_log || domain.log_size() > F::TWO_ADICITY {
        return None;
    }
    if !compatible_root::<F>(domain.log_size(), coefficient_log) {
        return None;
    }
    columns.end.checked_sub(columns.start)
}

fn compatible_root<F: TwoAdicField>(target_log: usize, coefficient_log: usize) -> bool {
    F::two_adic_generator(target_log).exp_power_of_2(target_log - coefficient_log)
        == F::two_adic_generator(coefficient_log)
}

fn select<F: TwoAdicField>(
    trace: &MaskedTrace<F>,
    columns: Range<usize>,
    length: usize,
) -> Option<RowMajorMatrix<F>> {
    let width = columns.end.checked_sub(columns.start)?;
    let mut values = Vec::new();
    values.try_reserve_exact(length).ok()?;
    for row in trace.coefficients.values.chunks_exact(trace.width()) {
        values.extend_from_slice(&row[columns.clone()]);
    }
    values.resize(length, F::ZERO);
    Some(RowMajorMatrix::new(values, width))
}
