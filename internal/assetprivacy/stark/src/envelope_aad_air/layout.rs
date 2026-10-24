// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{EPOCH, EnvelopeAadAir, KECCAK, OUTPUT, RHO, WIDTH};

impl EnvelopeAadAir {
    pub fn key_polynomials<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[..RHO])
    }
    pub fn rho_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[RHO..OUTPUT])
    }
    pub fn associated_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[OUTPUT..EPOCH])
    }
    pub fn epoch_bits<'a, T>(&self, row: &'a [T]) -> Option<&'a [T]> {
        (row.len() == WIDTH).then(|| &row[EPOCH..KECCAK])
    }
}
