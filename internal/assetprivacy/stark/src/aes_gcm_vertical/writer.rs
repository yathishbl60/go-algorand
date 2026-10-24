// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AesGcmAir, ROWS, WIDTH,
    witness::{Fixed, input, write_round},
};
use crate::{Value, aes_air::Byte, aes256_air::CipherBlock};
use p3_field::PrimeCharacteristicRing;

pub(crate) struct Encrypted {
    pub hash: [u8; 16],
    pub mask: [u8; 16],
    pub ciphertext: [u8; 512],
}

impl AesGcmAir {
    pub(crate) fn write_into(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        plaintext: [u8; 512],
        values: &mut [Value],
        width: usize,
    ) -> Option<Encrypted> {
        check_target(values.len(), width)?;
        let fixed = Fixed::new(key, nonce, plaintext);
        initialize(&fixed, values, width);
        Some(Encrypted {
            hash: core::array::from_fn(|i| byte(fixed.hash[i])),
            mask: core::array::from_fn(|i| byte(fixed.mask[i])),
            ciphertext: blocks(&fixed, values, width),
        })
    }
}

fn check_target(len: usize, width: usize) -> Option<()> {
    if width < WIDTH || !len.is_multiple_of(width) || len < ROWS.checked_mul(width)? {
        return None;
    }
    Some(())
}

fn initialize(fixed: &Fixed, values: &mut [Value], width: usize) {
    for (index, row) in values.chunks_exact_mut(width).enumerate() {
        if index < ROWS {
            fixed.write_fixed(&mut row[..WIDTH]);
            if index < 13 {
                fixed.write_key(row, index);
            }
        } else {
            fixed.write_core(row);
        }
    }
}

fn blocks(fixed: &Fixed, values: &mut [Value], width: usize) -> [u8; 512] {
    let mut ciphertext = [0; 512];
    for index in 0..34 {
        let block = CipherBlock::witness(&fixed.key, input(fixed.nonce, index));
        for round in 0..14 {
            let start = (13 + index * 14 + round) * width;
            write_round(&mut values[start..start + WIDTH], &block, round);
        }
        if index >= 2 {
            let start = (index - 2) * 16;
            for i in 0..16 {
                ciphertext[start + i] = fixed.plaintext[start + i] ^ byte(block.state[14][i]);
            }
        }
    }
    ciphertext
}

fn byte(bits: Byte<Value>) -> u8 {
    bits.iter()
        .enumerate()
        .fold(0, |v, (i, &bit)| v | (u8::from(bit == Value::ONE) << i))
}
