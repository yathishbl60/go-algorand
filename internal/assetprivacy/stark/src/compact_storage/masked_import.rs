// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::MaskedTrace;
use crate::compact::RANDOM_CODEWORDS;
use p3_field::TwoAdicField;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

impl<F: TwoAdicField> MaskedTrace<F> {
    /// Internal import of freshly randomized polynomials, never an entropy source.
    pub(crate) fn from_masked_coefficients(coefficients: RowMajorMatrix<F>) -> Option<Self> {
        let width = coefficients.width();
        if width <= RANDOM_CODEWORDS || !coefficients.values.len().is_multiple_of(width) {
            return None;
        }
        let rows = coefficients.height();
        if !rows.is_power_of_two() || !(1024..=131072).contains(&rows) {
            return None;
        }
        let shape = (rows / 2, width - RANDOM_CODEWORDS);
        Some(Self::from_coefficients(coefficients, shape))
    }
}
