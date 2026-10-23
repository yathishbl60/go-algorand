// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BITS, LEFT, OUTPUT, RIGHT, WIDTH};

/// Private bit references in transmitted MSB-first order; never publicize key material.
pub struct Wires<'a, T> {
    pub left: &'a [T],
    pub right: &'a [T],
    pub output: &'a [T],
}

/// Parent relations must bind these same private columns to their consumers.
pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    (row.len() == WIDTH).then(|| Wires {
        left: &row[LEFT..LEFT + BITS],
        right: &row[RIGHT..RIGHT + BITS],
        output: &row[OUTPUT..OUTPUT + BITS],
    })
}
