// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Borrow, CoefficientMatrix, MaskedTrace, Value32};
use p3_field::PrimeCharacteristicRing;

impl<S: Borrow<MaskedTrace<Value32>> + Send + Sync> CoefficientMatrix<S> {
    pub fn width(&self) -> usize {
        self.mask().width()
    }

    pub fn height(&self) -> usize {
        self.rows
    }

    /// Actual polynomial evaluation, O(coefficient_rows); use gather for queries.
    pub fn get(&self, row: usize, column: usize) -> Option<Value32> {
        if column >= self.width() {
            return None;
        }
        Some(self.polynomial(column, self.point(row)?))
    }

    /// Honest O(coefficient_rows*width) fallback, never a placeholder row.
    pub fn row(&self, row: usize) -> Option<Vec<Value32>> {
        let point = self.point(row)?;
        let mut values = Vec::new();
        values.try_reserve_exact(self.width()).ok()?;
        values.resize(self.width(), Value32::ZERO);
        self.evaluate_row(point, &mut values);
        Some(values)
    }

    fn evaluate_row(&self, point: Value32, values: &mut [Value32]) {
        for coefficients in self
            .mask()
            .coefficients()
            .values
            .chunks_exact(self.width())
            .rev()
        {
            for (value, coefficient) in values.iter_mut().zip(coefficients) {
                *value = *value * point + *coefficient;
            }
        }
    }

    pub(super) fn natural_row(&self, row: usize) -> Option<usize> {
        if row >= self.rows {
            return None;
        }
        Some(row.reverse_bits() >> (usize::BITS as usize - self.domain.log_size()))
    }

    fn point(&self, row: usize) -> Option<Value32> {
        Some(self.domain.element(self.natural_row(row)?))
    }

    fn polynomial(&self, column: usize, point: Value32) -> Value32 {
        self.mask()
            .coefficients()
            .values
            .chunks_exact(self.width())
            .rev()
            .fold(Value32::ZERO, |sum, coefficients| {
                sum * point + coefficients[column]
            })
    }
}
