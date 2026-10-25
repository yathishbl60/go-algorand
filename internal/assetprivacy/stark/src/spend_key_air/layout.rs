// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACCOUNT, COMMIT, CREATED, EPOCH, MESSAGE_BYTES, OWNER, ROLE, WIDTH};
use p3_keccak_air::NUM_KECCAK_COLS;

pub struct Wires<'a, T> {
    pub account_bits: &'a [T],
    pub epoch_bits: &'a [T],
    pub created_bits: &'a [T],
    pub owner_bits: &'a [T],
    pub commitment_limbs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        account_bits: message_bits(row, ACCOUNT, ROLE),
        epoch_bits: message_bits(row, EPOCH, CREATED),
        created_bits: message_bits(row, CREATED, OWNER),
        owner_bits: message_bits(row, OWNER, MESSAGE_BYTES),
        commitment_limbs: &row[COMMIT..COMMIT + 32],
    })
}

pub(super) fn message_bits<T>(row: &[T], first: usize, end: usize) -> &[T] {
    &row[NUM_KECCAK_COLS + first * 8..NUM_KECCAK_COLS + end * 8]
}
