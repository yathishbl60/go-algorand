// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ROTATIONS, ROUND_CONSTANTS};

pub(crate) fn theta(state: [u64; 25]) -> [u64; 25] {
    let parity: [u64; 5] =
        core::array::from_fn(|x| (0..5).fold(0, |sum, y| sum ^ state[5 * y + x]));
    core::array::from_fn(|index| {
        let x = index % 5;
        state[index] ^ parity[(x + 4) % 5] ^ parity[(x + 1) % 5].rotate_left(1)
    })
}

pub(crate) fn rho_pi(state: [u64; 25]) -> [u64; 25] {
    let mut output = [0; 25];
    for (index, value) in state.into_iter().enumerate() {
        let (x, y) = (index % 5, index / 5);
        output[5 * ((2 * x + 3 * y) % 5) + y] = value.rotate_left(ROTATIONS[index]);
    }
    output
}

pub(crate) fn chi_iota(state: [u64; 25], round: usize) -> Option<[u64; 25]> {
    let constant = *ROUND_CONSTANTS.get(round)?;
    let mut output = core::array::from_fn(|index| {
        let (x, y) = (index % 5, index / 5);
        state[index] ^ ((!state[5 * y + (x + 1) % 5]) & state[5 * y + (x + 2) % 5])
    });
    output[0] ^= constant;
    Some(output)
}
