// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later
use super::{
    AesGcmAir, PUBLIC_VALUES, ROWS, WIDTH,
    witness::{Fixed, input, write_round},
};
use crate::{Value, aes256_air::CipherBlock};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

impl AesGcmAir {
    /// Nonce bytes followed by 256 little-endian u16 ciphertext limbs.
    pub fn public_values(&self, nonce: [u8; 12], ciphertext: [u8; 512]) -> [Value; PUBLIC_VALUES] {
        core::array::from_fn(|i| {
            if i < 12 {
                return Value::from_u8(nonce[i]);
            }
            let offset = 2 * (i - 12);
            Value::from_u16(u16::from_le_bytes([
                ciphertext[offset],
                ciphertext[offset + 1],
            ]))
        })
    }

    /// Private AES trace only; no GHASH/KDF/KEM or proof authorization is implied.
    pub fn trace(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        plaintext: [u8; 512],
    ) -> RowMajorMatrix<Value> {
        Fixed::new(key, nonce, plaintext).trace()
    }
}

impl Fixed {
    fn trace(&self) -> RowMajorMatrix<Value> {
        let mut values = Value::zero_vec(ROWS * WIDTH);
        for (index, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
            self.write_fixed(row);
            if index < 13 {
                self.write_key(row, index);
            }
        }
        for block in 0..34 {
            self.write_block(&mut values, block);
        }
        RowMajorMatrix::new(values, WIDTH)
    }

    fn write_block(&self, values: &mut [Value], index: usize) {
        let block = CipherBlock::witness(&self.key, input(self.nonce, index));
        for round in 0..14 {
            let start = (13 + index * 14 + round) * WIDTH;
            write_round(&mut values[start..start + WIDTH], &block, round);
        }
    }
}
