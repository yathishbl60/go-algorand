// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{COEFFICIENTS, MlKemCiphertextAir, WIDTH};

impl MlKemCiphertextAir {
    /// Private u0/u1/u2/v scalar coefficients, each linked to public compression.
    pub fn coefficients<'a, T>(&self, row: &'a [T]) -> Option<&'a [T; COEFFICIENTS]> {
        if row.len() != WIDTH {
            return None;
        }
        row[..COEFFICIENTS].try_into().ok()
    }
}
