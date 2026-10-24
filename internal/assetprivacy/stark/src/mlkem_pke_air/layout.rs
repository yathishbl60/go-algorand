// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{GLOBAL_END, MESSAGE, NOISE, RHAT, WIDTH};

/// Private matrix/key/noise/message inputs for the enclosing KEM relation.
pub struct Wires<'a, T> {
    pub matrix: &'a [T],
    pub public_polynomials: &'a [T],
    pub noise: &'a [T],
    pub message_bits: &'a [T],
    pub outputs: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        matrix: &row[..2304],
        public_polynomials: &row[2304..RHAT],
        noise: &row[NOISE..MESSAGE],
        message_bits: &row[MESSAGE..MESSAGE + 256],
        outputs: &row[MESSAGE + 256..GLOBAL_END],
    })
}
