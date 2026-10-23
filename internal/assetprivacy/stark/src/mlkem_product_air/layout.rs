// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{LEFT, OUTPUT, RIGHT, WIDTH};

/// Private canonical scalar coefficient slices. Never serialize them as RPC data.
pub struct Wires<'a, T> {
    pub left: &'a [T],
    pub right: &'a [T],
    pub output: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        left: &row[LEFT..LEFT + 256],
        right: &row[RIGHT..RIGHT + 256],
        output: &row[OUTPUT..OUTPUT + 256],
    })
}
