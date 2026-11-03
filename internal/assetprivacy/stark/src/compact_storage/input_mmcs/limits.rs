// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::InputError;

/// Scalar budgets only: output is query_count times the sum of all matrix widths.
/// Salts, paths, vector metadata, FFT scratch and total RSS are separate.
#[derive(Clone, Copy, Debug)]
pub struct StreamedInputLimits {
    pub tile_columns: usize,
    pub max_tile_values: usize,
    pub max_output_values: usize,
}

impl StreamedInputLimits {
    pub fn validate(self) -> Result<(), InputError> {
        if self.tile_columns == 0 || self.max_tile_values == 0 || self.max_output_values == 0 {
            return Err(InputError::Limits);
        }
        Ok(())
    }
}
