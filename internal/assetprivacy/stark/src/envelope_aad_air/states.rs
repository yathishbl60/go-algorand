// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::witness::Inputs;
use p3_keccak::KeccakF;
use p3_symmetric::Permutation;

pub(super) fn permutation_inputs(input: &Inputs) -> Vec<[u64; 25]> {
    let mut padded = [0; 2592];
    padded[..256].copy_from_slice(&input.context);
    padded[256..1440].copy_from_slice(&input.key);
    padded[1440..2528].copy_from_slice(&input.ciphertext);
    padded[2528] = 6;
    padded[2591] = 128;
    let mut state = [0; 25];
    let mut states = Vec::with_capacity(36);
    for block in padded.as_chunks::<72>().0 {
        absorb(&mut state, block);
        states.push(state);
        KeccakF {}.permute_mut(&mut state);
    }
    states
}

fn absorb(state: &mut [u64; 25], block: &[u8]) {
    for (index, &byte) in block.iter().enumerate() {
        state[index / 8] ^= u64::from(byte) << (8 * (index % 8));
    }
}
