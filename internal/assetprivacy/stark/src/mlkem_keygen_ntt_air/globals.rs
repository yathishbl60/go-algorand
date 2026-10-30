// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CORE, ROWS, TRANSFORMED, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn write_globals(
    values: &mut [Value],
    target: (usize, usize, usize),
    root: &[Value; CORE],
    outputs: &[[u16; 256]; 6],
) {
    for parent in values.chunks_exact_mut(target.0).skip(target.2).take(ROWS) {
        let row = &mut parent[target.1..target.1 + WIDTH];
        row[..CORE].copy_from_slice(root);
        for (to, &value) in row[TRANSFORMED..WIDTH]
            .iter_mut()
            .zip(outputs.iter().flatten())
        {
            *to = Value::from_u16(value);
        }
    }
}
