// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CompactKeccakAir, ROUNDS, STATE_BITS, WIDTH};
use crate::serial_keccak::{chi_iota, rho_pi, theta};
use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

impl<F: PrimeField64> CompactKeccakAir<F> {
    /// One independent private Keccak-f permutation per input; no sponge binding.
    pub fn trace(&self, inputs: &[[u64; 25]]) -> Option<RowMajorMatrix<F>> {
        if inputs.len() != self.count {
            return None;
        }
        let mut values = Vec::new();
        values.try_reserve_exact(self.rows() * WIDTH).ok()?;
        values.resize(self.rows() * WIDTH, F::ZERO);
        for (permutation, &input) in inputs.iter().enumerate() {
            let start = permutation * ROUNDS * WIDTH;
            write_permutation(input, &mut values[start..start + ROUNDS * WIDTH])?;
        }
        Some(RowMajorMatrix::new(values, WIDTH))
    }
}

fn write_permutation<F: PrimeField64>(mut state: [u64; 25], rows: &mut [F]) -> Option<()> {
    for round in 0..ROUNDS {
        let after = theta(state);
        let row = &mut rows[round * WIDTH..(round + 1) * WIDTH];
        for (index, cell) in row[..STATE_BITS].iter_mut().enumerate() {
            *cell = F::from_bool(after[index / 64] & (1u64 << (index % 64)) != 0);
        }
        let parity: [u64; 5] =
            core::array::from_fn(|x| (0..5).fold(0, |value, y| value ^ state[5 * y + x]));
        for (index, cell) in row[STATE_BITS..].iter_mut().enumerate() {
            *cell = F::from_bool(parity[index / 64] & (1u64 << (index % 64)) != 0);
        }
        state = chi_iota(rho_pi(after), round)?;
    }
    Some(())
}
