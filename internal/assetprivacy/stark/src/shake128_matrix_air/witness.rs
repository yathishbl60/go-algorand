// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{RATE_BYTES, Shake128MatrixAir};
use p3_keccak::KeccakF;
use p3_keccak_air::NUM_ROUNDS;
use p3_symmetric::Permutation;

impl Shake128MatrixAir {
    pub(super) fn inputs(&self, seed: [u8; 32]) -> (Vec<[u64; 25]>, Vec<u8>) {
        let mut state = initial(seed, self.first, self.second);
        let mut inputs = Vec::with_capacity(self.rows / NUM_ROUNDS);
        let mut output = Vec::with_capacity(self.output_bytes());
        for _ in 0..self.blocks {
            inputs.push(state);
            KeccakF {}.permute_mut(&mut state);
            for byte in 0..RATE_BYTES {
                output.push((state[byte / 8] >> (8 * (byte % 8))) as u8);
            }
        }
        inputs.resize(self.rows / NUM_ROUNDS, [0; 25]);
        (inputs, output)
    }
}

fn initial(seed: [u8; 32], first: u8, second: u8) -> [u64; 25] {
    let mut state = [0u64; 25];
    for (index, byte) in seed.into_iter().enumerate() {
        state[index / 8] |= u64::from(byte) << (8 * (index % 8));
    }
    state[4] = u64::from(first) | (u64::from(second) << 8) | (0x1fu64 << 16);
    state[20] = 0x80u64 << 56;
    state
}
