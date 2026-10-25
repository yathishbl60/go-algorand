// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, EPOCH, RHO, WIDTH, WORK};

pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub rho_bits: &'a [T],
    pub epoch_bits: &'a [T],
    pub account_bits: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[..RHO],
        rho_bits: &row[RHO..EPOCH],
        epoch_bits: &row[EPOCH..ACCOUNT],
        account_bits: &row[ACCOUNT..WORK],
    })
}
