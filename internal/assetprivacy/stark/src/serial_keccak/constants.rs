// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

pub(crate) const RC_POSITIONS: [usize; 7] = [0, 1, 3, 7, 15, 31, 63];
pub(crate) const ROUND_CONSTANTS: [u64; 24] = p3_keccak_air::RC;
pub(crate) const ROTATIONS: [u32; 25] = rotations();

// Keccak's triangular offsets along its fixed Pi orbit, in y-major lane order.
const fn rotations() -> [u32; 25] {
    let mut offsets = [0; 25];
    let (mut x, mut y, mut step) = (1usize, 0usize, 0u32);
    while step < 24 {
        offsets[5 * y + x] = ((step + 1) * (step + 2) / 2) % 64;
        let next = (y, (2 * x + 3 * y) % 5);
        x = next.0;
        y = next.1;
        step += 1;
    }
    offsets
}
