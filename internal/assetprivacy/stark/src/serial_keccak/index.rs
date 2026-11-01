// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ROTATIONS, STATE_BITS};

// Flat sponge bits are little-endian within lanes, with lane index 5*y+x.
pub(crate) const fn state_index(x: usize, y: usize, z: usize) -> usize {
    64 * (5 * y + x) + z
}

pub(crate) const fn parity_index(x: usize, z: usize) -> usize {
    STATE_BITS + 64 * x + z
}

pub(super) const fn coordinates(index: usize) -> (usize, usize, usize) {
    ((index / 64) % 5, index / 320, index % 64)
}

pub(super) const fn rho_source(index: usize) -> usize {
    let (x, y, z) = coordinates(index);
    let (a, b) = ((x + 3 * y) % 5, x);
    state_index(a, b, (z + 64 - ROTATIONS[5 * b + a] as usize) % 64)
}
