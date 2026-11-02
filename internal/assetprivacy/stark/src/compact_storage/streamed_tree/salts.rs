// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{SALT_ELEMS, Value32};
use rand::{CryptoRng, RngExt};

pub(super) fn draw<R: CryptoRng>(
    rows: usize,
    matrices: usize,
    rng: &mut R,
) -> Option<Vec<Vec<[Value32; SALT_ELEMS]>>> {
    rows.checked_mul(matrices)?.checked_mul(SALT_ELEMS)?;
    let mut all = Vec::new();
    all.try_reserve_exact(matrices).ok()?;
    for _ in 0..matrices {
        let mut matrix = Vec::new();
        matrix.try_reserve_exact(rows).ok()?;
        for _ in 0..rows {
            matrix.push(core::array::from_fn(|_| rng.random()));
        }
        all.push(matrix);
    }
    Some(all)
}
