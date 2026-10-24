// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BUFFER, CURSOR, MlKemStreamMatrixAir, OUTPUT, SEED, WIDTH};

impl MlKemStreamMatrixAir {
    /// Private little-endian rho bits; an enclosing AIR must bind them.
    pub fn seed_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != WIDTH {
            return None;
        }
        row[SEED..BUFFER].try_into().ok()
    }
    /// Private first 256 accepted canonical SampleNTT residues.
    pub fn coefficients<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; 256]> {
        if row.len() != WIDTH {
            return None;
        }
        row[OUTPUT..CURSOR].try_into().ok()
    }
}
