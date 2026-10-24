// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, GH_ROW, GH_START, GcmVerticalAir, PUBLIC_VALUES, ROWS, WIDTH};
use crate::{
    Value,
    aes_gcm_vertical::{AesGcmAir, Encrypted},
    ghash_chain::GhashChainAir,
};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::dense::RowMajorMatrix;

pub struct GcmWitness {
    pub trace: RowMajorMatrix<Value>,
    pub public_values: [Value; PUBLIC_VALUES],
}

impl GcmVerticalAir {
    /// Constructs private columns and public nonce/ciphertext/tag; no proof runs.
    pub fn trace(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        associated: [u8; 64],
        plaintext: [u8; 512],
    ) -> Option<GcmWitness> {
        let mut values = Value::zero_vec(ROWS * WIDTH);
        let encrypted = AesGcmAir.write_into(key, nonce, plaintext, &mut values, WIDTH)?;
        finish(values, nonce, associated, encrypted)
    }

    pub fn public_values(
        &self,
        nonce: [u8; 12],
        ciphertext: [u8; 512],
        tag: [u8; 16],
    ) -> [Value; PUBLIC_VALUES] {
        let aes = AesGcmAir.public_values(nonce, ciphertext);
        core::array::from_fn(|i| {
            if i < 268 {
                return aes[i];
            }
            let offset = (i - 268) * 2;
            Value::from_u16(u16::from_le_bytes([tag[offset], tag[offset + 1]]))
        })
    }
}

fn finish(
    mut values: Vec<Value>,
    nonce: [u8; 12],
    associated: [u8; 64],
    encrypted: Encrypted,
) -> Option<GcmWitness> {
    let hash = hash(&encrypted, associated, &mut values)?;
    let tag = core::array::from_fn(|i| hash[i] ^ encrypted.mask[i]);
    for row in values.as_chunks_mut::<WIDTH>().0 {
        associated_bits(&mut row[AAD..], &associated);
    }
    Some(GcmWitness {
        trace: RowMajorMatrix::new(values, WIDTH),
        public_values: GcmVerticalAir.public_values(nonce, encrypted.ciphertext, tag),
    })
}

fn hash(encrypted: &Encrypted, associated: [u8; 64], values: &mut [Value]) -> Option<[u8; 16]> {
    GhashChainAir.write_into(
        encrypted.hash,
        associated,
        encrypted.ciphertext,
        values,
        (WIDTH, GH_ROW, GH_START),
    )
}

fn associated_bits(row: &mut [Value], input: &[u8; 64]) {
    for (target, &source) in row.as_chunks_mut::<8>().0.iter_mut().zip(input) {
        for (bit, value) in target.iter_mut().enumerate() {
            *value = Value::from_bool(source & (1 << bit) != 0);
        }
    }
}
