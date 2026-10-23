// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{H, KEY, MASK, PT, WIDTH};

/// All wires are private and use byte-order concatenation with LSB-first byte bits.
pub struct Wires<'a, T> {
    pub key: &'a [T],
    pub plaintext: &'a [T],
    pub hash_subkey: &'a [T],
    pub tag_mask: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        key: &row[KEY..KEY + 256], plaintext: &row[PT..PT + 4096],
        hash_subkey: &row[H..H + 128], tag_mask: &row[MASK..MASK + 128],
    })
}
