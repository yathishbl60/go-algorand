// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MlKemNoiseAir, witness::auxiliary};
use crate::Value;
use p3_air::BaseAir;
use p3_field::PrimeCharacteristicRing;
use p3_keccak::KeccakF;
use p3_matrix::{Matrix, dense::RowMajorMatrix};
use p3_symmetric::Permutation;

impl MlKemNoiseAir {
    pub fn trace(&self, seed: [u8; 32]) -> Option<RowMajorMatrix<Value>> {
        let message = self.message(seed);
        let hash = self.hash.trace(&message)?;
        Some(self.join(hash, auxiliary(output(&message))))
    }

    fn join(&self, hash: RowMajorMatrix<Value>, auxiliary: [Value; 2624]) -> RowMajorMatrix<Value> {
        let width = <Self as BaseAir<Value>>::width(self);
        let mut values = Value::zero_vec(hash.height() * width);
        for (r, row) in values.chunks_exact_mut(width).enumerate() {
            let start = self.digest_start();
            row[..start].copy_from_slice(&hash.values[r * start..(r + 1) * start]);
            row[start..].copy_from_slice(&auxiliary);
        }
        RowMajorMatrix::new(values, width)
    }
}

pub(super) fn output(message: &[u8; 33]) -> [u8; 128] {
    let mut state = [0u64; 25];
    for (i, byte) in message.iter().enumerate() {
        state[i / 8] ^= u64::from(*byte) << (8 * (i % 8));
    }
    state[4] ^= 0x1f << 8; // Domain suffix at byte 33, after the nonce.
    state[16] ^= 0x80 << 56; // Last rate byte 135, SHAKE256 rate 136.
    KeccakF {}.permute_mut(&mut state);
    core::array::from_fn(|i| (state[i / 8] >> (8 * (i % 8))) as u8)
}
