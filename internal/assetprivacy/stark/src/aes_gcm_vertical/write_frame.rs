// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    ROWS, WIDTH,
    witness::{Fixed, input, write_round},
};
use crate::{Value, aes_air::Byte, aes256_air::CipherBlock};
use p3_field::PrimeCharacteristicRing;

pub(super) fn initialize(fixed: &Fixed, values: &mut [Value], shape: (usize, usize, usize, usize)) {
    let (stride, start, first, rows) = shape;
    for (index, parent) in values
        .chunks_exact_mut(stride)
        .skip(first)
        .take(rows)
        .enumerate()
    {
        let row = &mut parent[start..start + WIDTH];
        row.fill(Value::ZERO);
        if index < ROWS {
            fixed.write_fixed(row);
            if index < 13 {
                fixed.write_key(row, index);
            }
        } else {
            fixed.write_core(row);
        }
    }
}

pub(super) fn blocks(
    fixed: &Fixed,
    values: &mut [Value],
    shape: (usize, usize, usize, usize),
) -> [u8; 512] {
    let (stride, column, first, _) = shape;
    let mut ciphertext = [0; 512];
    for index in 0..34 {
        let block = CipherBlock::witness(&fixed.key, input(fixed.nonce, index));
        for round in 0..14 {
            let start = (first + 13 + index * 14 + round) * stride + column;
            write_round(&mut values[start..start + WIDTH], &block, round);
        }
        if index >= 2 {
            output(&mut ciphertext, fixed, &block, index);
        }
    }
    ciphertext
}

fn output(output: &mut [u8; 512], fixed: &Fixed, block: &CipherBlock<Value>, index: usize) {
    let start = (index - 2) * 16;
    for i in 0..16 {
        output[start + i] = fixed.plaintext[start + i] ^ byte(block.state[14][i]);
    }
}

pub(super) fn byte(bits: Byte<Value>) -> u8 {
    bits.iter()
        .enumerate()
        .fold(0, |v, (i, &bit)| v | (u8::from(bit == Value::ONE) << i))
}
