// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::witness::Inputs;
use p3_keccak::KeccakF;
use p3_symmetric::Permutation;

pub(super) fn permutation_inputs(input: &Inputs) -> Vec<[u64; 25]> {
    let mut padded = [0; 1224];
    padded[..1184].copy_from_slice(&input.key);
    padded[1184] = 6;
    padded[1223] = 128;
    let mut state = [0; 25];
    let mut states = Vec::with_capacity(10);
    for block in padded.as_chunks::<136>().0 {
        absorb(&mut state, block);
        states.push(state);
        KeccakF {}.permute_mut(&mut state);
    }
    states.push(expand_input(input));
    states
}

fn expand_input(input: &Inputs) -> [u64; 25] {
    let mut padded = [0; 72];
    padded[..64].copy_from_slice(&input.message);
    padded[64] = 6;
    padded[71] = 128;
    let mut state = [0; 25];
    absorb(&mut state, &padded);
    state
}

fn absorb(state: &mut [u64; 25], block: &[u8]) {
    for (index, &byte) in block.iter().enumerate() {
        state[index / 8] ^= u64::from(byte) << (8 * (index % 8));
    }
}
