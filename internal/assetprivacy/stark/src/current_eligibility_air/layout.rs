// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, LEAF, REVISION, UPDATED, WIDTH, WORK};

pub struct Wires<'a, T> {
    pub account_bits: &'a [T],
    pub revision_bits: &'a [T],
    pub updated_bits: &'a [T],
    pub leaf_limbs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        account_bits: &row[ACCOUNT..REVISION],
        revision_bits: &row[REVISION..UPDATED],
        updated_bits: &row[UPDATED..LEAF],
        leaf_limbs: &row[LEAF..WORK],
    })
}
