// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{HASH, MESSAGE, MlKemStreamDerivationAir, OUTPUT, RHO, WIDTH};

impl MlKemStreamDerivationAir {
    pub fn key_polynomials<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[..RHO])
    }
    pub fn rho_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[RHO..MESSAGE])
    }
    pub fn message_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[MESSAGE..HASH])
    }
    pub fn shared_secret_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[OUTPUT..OUTPUT + 256])
    }
    pub fn coins_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[OUTPUT + 256..OUTPUT + 512])
    }
}
