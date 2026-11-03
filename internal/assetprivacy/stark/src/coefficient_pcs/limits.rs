// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Error;
use crate::compact_storage::StreamedInputLimits;

/// Per-buffer scalar caps, not an aggregate RAM guarantee.
/// Extension-field claims count all ten base-field coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub tile_columns: usize,
    pub max_tile_values: usize,
    pub max_coefficient_values: usize,
    pub max_evaluation_values: usize,
    pub max_claim_values: usize,
    pub max_query_values: usize,
}

impl Limits {
    pub fn validate(self) -> Result<(), Error> {
        let values = [
            self.tile_columns,
            self.max_tile_values,
            self.max_coefficient_values,
            self.max_evaluation_values,
            self.max_claim_values,
            self.max_query_values,
        ];
        if values.contains(&0) {
            return Err(Error::Limits);
        }
        Ok(())
    }

    pub(crate) fn input(self) -> StreamedInputLimits {
        StreamedInputLimits {
            tile_columns: self.tile_columns,
            max_tile_values: self.max_tile_values,
            max_output_values: self.max_query_values,
        }
    }
}
