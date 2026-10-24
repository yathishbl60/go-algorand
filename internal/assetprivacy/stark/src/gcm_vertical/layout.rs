// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, PT, WIDTH};

/// Constant private byte-order wires, with LSB-first bits within each byte.
pub struct Wires<'a, T> {
    pub key: &'a [T],
    pub plaintext: &'a [T],
    pub associated: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        key: &row[..256],
        plaintext: &row[PT..PT + 4096],
        associated: &row[AAD..AAD + 512],
    })
}
