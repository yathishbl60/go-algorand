// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fixed 128-byte SHAKE256 output for ML-KEM768 noise sampling.
//! Longer XOF squeezing and ML-KEM matrix generation remain separate.

use crate::{Value, sha3_air::Sha3Air};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeCharacteristicRing;
use p3_keccak_air::NUM_KECCAK_COLS;
use p3_matrix::dense::RowMajorMatrix;

pub const OUTPUT_BYTES: usize = 128;
pub const PUBLIC_VALUES: usize = OUTPUT_BYTES / 2;

#[derive(Clone, Copy)]
pub struct Shake256Air {
    hash: Sha3Air,
}

impl Shake256Air {
    pub fn new(message_len: usize) -> Option<Self> {
        Some(Self {
            hash: Sha3Air::shake256_128(message_len)?,
        })
    }
    pub fn trace_rows(&self) -> usize {
        self.hash.trace_rows()
    }
    pub fn trace(&self, message: &[u8]) -> Option<RowMajorMatrix<Value>> {
        self.hash.trace(message)
    }
    pub fn message_bit_column(&self, byte: usize, bit: usize) -> Option<usize> {
        (byte < self.hash.message_bytes() && bit < 8).then(|| NUM_KECCAK_COLS + byte * 8 + bit)
    }
}

impl BaseAir<Value> for Shake256Air {
    fn width(&self) -> usize {
        self.hash.width()
    }
    fn num_public_values(&self) -> usize {
        PUBLIC_VALUES
    }
}

impl<AB: AirBuilder<F = Value>> Air<AB> for Shake256Air {
    fn eval(&self, builder: &mut AB) {
        self.hash.eval(builder);
    }
}

pub fn output_values(output: [u8; OUTPUT_BYTES]) -> [Value; PUBLIC_VALUES] {
    core::array::from_fn(|limb| {
        Value::from_u16(u16::from_le_bytes([output[2 * limb], output[2 * limb + 1]]))
    })
}
