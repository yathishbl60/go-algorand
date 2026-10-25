// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AuthorizedDisclosureAir, EPOCH, OPENING, RHO, WIDTH, WORK};

pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub rho_bits: &'a [T],
    pub epoch_bits: &'a [T],
    pub opening_bits: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[..RHO],
        rho_bits: &row[RHO..EPOCH],
        epoch_bits: &row[EPOCH..OPENING],
        opening_bits: &row[OPENING..WORK],
    })
}

impl AuthorizedDisclosureAir {
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        let opening = wires(row)?.opening_bits;
        Some(core::array::from_fn(|bit| {
            opening[(176 - bit / 8) * 8 + bit % 8]
        }))
    }
}
