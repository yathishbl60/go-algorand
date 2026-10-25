// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, COMMIT, OWNER, SERIAL, VALUE, WIDTH, WORK};

pub struct Wires<'a, T> {
    pub account_bits: &'a [T],
    pub value_bits: &'a [T],
    pub serial_bits: &'a [T],
    pub owner_limbs: &'a [T],
    pub commitment_limbs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        account_bits: &row[ACCOUNT..VALUE],
        value_bits: &row[VALUE..SERIAL],
        serial_bits: &row[SERIAL..OWNER],
        owner_limbs: &row[OWNER..COMMIT],
        commitment_limbs: &row[COMMIT..WORK],
    })
}
