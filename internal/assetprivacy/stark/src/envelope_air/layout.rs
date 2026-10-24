// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, COMMIT, EPOCH, EnvelopeAir, NONZERO, OPENING, RHO, SHARED, WIDTH};

/// Private recipient-key and committed-note wires for enclosing authorization.
pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub rho_bits: &'a [T],
    pub opening_bits: &'a [T],
    pub shared_secret_bits: &'a [T],
    pub epoch_bits: &'a [T],
    pub commitment_limbs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[..RHO],
        rho_bits: &row[RHO..SHARED],
        shared_secret_bits: &row[SHARED..AAD],
        opening_bits: &row[OPENING..COMMIT],
        commitment_limbs: &row[COMMIT..EPOCH],
        epoch_bits: &row[EPOCH..NONZERO],
    })
}

impl EnvelopeAir {
    /// Exact BE-encoded note amount, returned as least-significant-first bits.
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        let opening = wires(row)?.opening_bits;
        Some(core::array::from_fn(|bit| {
            opening[(176 - bit / 8) * 8 + bit % 8]
        }))
    }
}
