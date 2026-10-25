// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ACCOUNT, AuthorizedRecipientAir, OWNER, RHO, SPEND_EPOCH, VALUE, VIEW_EPOCH, WIDTH, WORK,
};

pub struct Wires<'a, T> {
    pub public_polynomials: &'a [T],
    pub rho_bits: &'a [T],
    pub view_epoch_bits: &'a [T],
    pub account_bits: &'a [T],
    pub owner_bits: &'a [T],
    pub spend_epoch_bits: &'a [T],
    pub value_bits: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        public_polynomials: &row[..RHO],
        rho_bits: &row[RHO..VIEW_EPOCH],
        view_epoch_bits: &row[VIEW_EPOCH..ACCOUNT],
        account_bits: &row[ACCOUNT..OWNER],
        owner_bits: &row[OWNER..SPEND_EPOCH],
        spend_epoch_bits: &row[SPEND_EPOCH..VALUE],
        value_bits: &row[VALUE..WORK],
    })
}

impl AuthorizedRecipientAir {
    pub fn value_bits<T: Copy>(&self, row: &[T]) -> Option<[T; 64]> {
        wires(row)?.value_bits.try_into().ok()
    }
}
