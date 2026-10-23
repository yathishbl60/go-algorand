// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{INPUT, OUTPUT, WIDTH};

/// Private scalar coefficients in canonical 0..3328 order, not packed bytes.
pub struct Wires<'a, T> {
    pub input: &'a [T],
    pub output: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        input: &row[INPUT..INPUT + 256],
        output: &row[OUTPUT..OUTPUT + 256],
    })
}
