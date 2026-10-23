// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{ACTIVE_ROWS, BITS, GhashChainAir, PUBLIC_VALUES, ROWS, WIDTH};
use super::{witness_state::State, witness_write};
use crate::Value;
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

struct Inputs {
    h: u128,
    associated: [u8; 64],
    ciphertext: [u8; 512],
}

impl GhashChainAir {
    pub fn public_values(&self, ciphertext: [u8; 512]) -> [Value; PUBLIC_VALUES] {
        core::array::from_fn(|i| {
            Value::from_u16(u16::from_le_bytes([
                ciphertext[2 * i],
                ciphertext[2 * i + 1],
            ]))
        })
    }

    /// Builds private columns; never proves or authorizes the enclosing encryption.
    pub fn trace(
        &self,
        h: [u8; 16],
        associated: [u8; 64],
        ciphertext: [u8; 512],
    ) -> RowMajorMatrix<Value> {
        Inputs {
            h: u128::from_be_bytes(h),
            associated,
            ciphertext,
        }
        .trace()
    }
}

impl Inputs {
    fn trace(&self) -> RowMajorMatrix<Value> {
        let result = self.result();
        let mut state = State::new();
        let mut values = Value::zero_vec(ROWS * WIDTH);
        for (round, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            self.prepare(&mut state, round);
            witness_write::fixed(row, self.h, &self.associated, result);
            witness_write::state(row, &state, round);
            if round < ACTIVE_ROWS {
                state.advance(round % BITS);
            }
        }
        RowMajorMatrix::new(values, WIDTH)
    }

    fn prepare(&self, state: &mut State, round: usize) {
        if round < ACTIVE_ROWS && round.is_multiple_of(BITS) {
            state.start(self.h, self.block(round / BITS));
        }
    }

    fn result(&self) -> u128 {
        let mut state = State::new();
        for round in 0..ACTIVE_ROWS {
            self.prepare(&mut state, round);
            state.advance(round % BITS);
        }
        state.y
    }

    fn block(&self, block: usize) -> u128 {
        if block < 4 {
            return u128::from_be_bytes(
                self.associated[block * 16..(block + 1) * 16]
                    .try_into()
                    .expect("AAD block"),
            );
        }
        if block < 36 {
            let start = (block - 4) * 16;
            return u128::from_be_bytes(
                self.ciphertext[start..start + 16]
                    .try_into()
                    .expect("ciphertext block"),
            );
        }
        (512u128 << 64) | 4096
    }
}
