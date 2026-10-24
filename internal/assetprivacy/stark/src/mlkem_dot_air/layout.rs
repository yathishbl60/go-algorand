// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE, MlKemDotAir, OUTPUT, RHAT, WIDTH};

impl MlKemDotAir {
    pub fn matrix<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[..2304])
    }
    pub fn public_polynomials<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[2304..RHAT])
    }
    pub fn r_hat<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[RHAT..OUTPUT])
    }
    pub fn outputs<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[OUTPUT..CORE])
    }
}
