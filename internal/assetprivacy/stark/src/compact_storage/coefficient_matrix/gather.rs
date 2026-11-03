// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Borrow, CoefficientMatrix, MaskedTrace, Value32};
use p3_dft::Radix2Dit;
use p3_field::PrimeCharacteristicRing;

impl<S: Borrow<MaskedTrace<Value32>> + Send + Sync> CoefficientMatrix<S> {
    /// FFT each column tile once; preserve bit-reversed query order and duplicates.
    /// Budgets count scalar tile/output cells, not FFT scratch or total RSS.
    pub fn gather(
        &self,
        indices: &[usize],
        tile_columns: usize,
        max_tile_values: usize,
        max_output_values: usize,
        dft: &Radix2Dit<Value32>,
    ) -> Option<Vec<Vec<Value32>>> {
        self.validate_gather(indices, tile_columns, max_tile_values, max_output_values)?;
        if indices.is_empty() {
            return Some(Vec::new());
        }
        let mut output = allocate(indices.len(), self.width())?;
        self.gather_into(indices, tile_columns, max_tile_values, dft, &mut output)?;
        Some(output)
    }

    fn validate_gather(
        &self,
        indices: &[usize],
        columns: usize,
        tile: usize,
        output: usize,
    ) -> Option<()> {
        if columns == 0 || indices.iter().any(|index| *index >= self.rows) {
            return None;
        }
        let cells = indices.len().checked_mul(self.width())?;
        let tile_cells = if indices.is_empty() {
            0
        } else {
            self.rows.checked_mul(columns.min(self.width()))?
        };
        (cells <= output && tile_cells <= tile).then_some(())
    }

    fn gather_into(
        &self,
        indices: &[usize],
        columns: usize,
        budget: usize,
        dft: &Radix2Dit<Value32>,
        output: &mut [Vec<Value32>],
    ) -> Option<()> {
        for start in (0..self.width()).step_by(columns) {
            let end = start + columns.min(self.width() - start);
            let tile = self
                .mask()
                .evaluate_columns(start..end, self.domain, budget, dft)?;
            for (row, &query) in output.iter_mut().zip(indices) {
                let from = self.natural_row(query)?.checked_mul(tile.width)?;
                row[start..end].copy_from_slice(&tile.values[from..from + tile.width]);
            }
        }
        Some(())
    }
}

fn allocate(count: usize, width: usize) -> Option<Vec<Vec<Value32>>> {
    let mut output = Vec::new();
    output.try_reserve_exact(count).ok()?;
    for _ in 0..count {
        let mut row = Vec::new();
        row.try_reserve_exact(width).ok()?;
        row.resize(width, Value32::ZERO);
        output.push(row);
    }
    Some(output)
}
