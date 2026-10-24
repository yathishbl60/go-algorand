// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_keccak::KeccakF;
use p3_keccak_air::NUM_ROUNDS;
use p3_symmetric::Permutation;

use super::Sha3Air;

impl Sha3Air {
    pub(super) fn permutation_inputs(&self, message: &[u8]) -> Vec<[u64; 25]> {
        let mut padded = vec![0u8; self.blocks() * self.rate_bytes()];
        padded[..message.len()].copy_from_slice(message);
        padded[message.len()] = self.domain_suffix;
        *padded.last_mut().expect("one padding block") |= 0x80;
        let mut state = [0u64; 25];
        let mut inputs = Vec::with_capacity(self.blocks());
        for block in padded.chunks_exact(self.rate_bytes()) {
            absorb(&mut state, block);
            inputs.push(state);
            KeccakF {}.permute_mut(&mut state);
        }
        inputs.resize(inputs.len().max(self.trace_rows() / NUM_ROUNDS), [0; 25]);
        inputs
    }
}

fn absorb(state: &mut [u64; 25], block: &[u8]) {
    for (i, byte) in block.iter().enumerate() {
        state[i / 8] ^= (*byte as u64) << (8 * (i % 8));
    }
}
