// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RHO, SIGMA};
use p3_keccak_air::NUM_KECCAK_COLS;

pub const WIDTH: usize = SIGMA + 256;

pub struct Wires<'a, T> {
    pub seed: &'a [T],
    pub rho: &'a [T],
    pub sigma: &'a [T],
}

pub fn wires<T>(row: &[T]) -> Option<Wires<'_, T>> {
    if row.len() != WIDTH {
        return None;
    }
    Some(Wires {
        seed: &row[NUM_KECCAK_COLS..NUM_KECCAK_COLS + 256],
        rho: &row[RHO..SIGMA],
        sigma: &row[SIGMA..WIDTH],
    })
}
