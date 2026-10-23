// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, BITS, H, RESULT, WIDTH};

/// Private wires use transmitted MSB-first bits, including each AAD byte.
pub struct Wires<'a, T> {
    pub hash_subkey: &'a [T],
    pub associated: &'a [T],
    pub result: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        hash_subkey: &row[H..H + BITS],
        associated: &row[AAD..AAD + 512],
        result: &row[RESULT..RESULT + BITS],
    })
}
