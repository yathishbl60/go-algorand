// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, CREATED, EPOCH, LEAF, OWNER, RHO, WIDTH, WORK};

pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub account_bits: &'a [T],
    pub epoch_bits: &'a [T],
    pub created_bits: &'a [T],
    pub rho_bits: &'a [T],
    pub owner_bits: &'a [T],
    pub leaf_limbs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[..ACCOUNT],
        account_bits: &row[ACCOUNT..EPOCH],
        epoch_bits: &row[EPOCH..CREATED],
        created_bits: &row[CREATED..RHO],
        rho_bits: &row[RHO..OWNER],
        owner_bits: &row[OWNER..LEAF],
        leaf_limbs: &row[LEAF..WORK],
    })
}
