// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{AAD, FRAME_ROWS, GH_ROW, GH_START, GcmVerticalAir, WIDTH};
use crate::{
    Value,
    aes_gcm_vertical::{AesGcmAir, Encrypted},
    ghash_chain::GhashChainAir,
};
use p3_field::PrimeCharacteristicRing;

type Output = ([u8; 512], [u8; 16]);
type Inputs = ([u8; 32], [u8; 12], [u8; 64], [u8; 512]);

impl GcmVerticalAir {
    pub(crate) fn write_into(
        &self,
        key: [u8; 32],
        nonce: [u8; 12],
        associated: [u8; 64],
        plaintext: [u8; 512],
        values: &mut [Value],
        shape: (usize, usize, usize),
    ) -> Option<Output> {
        let target = checked_target(values, shape)?;
        write(
            (key, nonce, associated, plaintext),
            target,
            shape.0,
            shape.1,
        )
    }
}

fn checked_target(values: &mut [Value], shape: (usize, usize, usize)) -> Option<&mut [Value]> {
    let (stride, start, first) = shape;
    if stride == 0 || !values.len().is_multiple_of(stride) || start.checked_add(WIDTH)? > stride {
        return None;
    }
    if first.checked_add(FRAME_ROWS)? > values.len() / stride {
        return None;
    }
    Some(&mut values[first * stride..(first + FRAME_ROWS) * stride])
}

fn write(input: Inputs, values: &mut [Value], stride: usize, start: usize) -> Option<Output> {
    let associated = input.2;
    let encrypted = encrypt(input, values, stride, start)?;
    let hash = hash(&encrypted, associated, values, (stride, start))?;
    for row in values.chunks_exact_mut(stride) {
        associated_bits(&mut row[start + AAD..start + WIDTH], &associated);
    }
    let tag = core::array::from_fn(|i| hash[i] ^ encrypted.mask[i]);
    Some((encrypted.ciphertext, tag))
}

fn encrypt(input: Inputs, values: &mut [Value], stride: usize, start: usize) -> Option<Encrypted> {
    AesGcmAir.write_frame(
        input.0,
        input.1,
        input.3,
        values,
        (stride, start, 0, FRAME_ROWS),
    )
}

fn hash(
    encrypted: &Encrypted,
    associated: [u8; 64],
    values: &mut [Value],
    shape: (usize, usize),
) -> Option<[u8; 16]> {
    GhashChainAir.write_into(
        encrypted.hash,
        associated,
        encrypted.ciphertext,
        values,
        (shape.0, GH_ROW, shape.1 + GH_START),
    )
}

fn associated_bits(row: &mut [Value], input: &[u8; 64]) {
    for (bit, target) in row.iter_mut().enumerate() {
        *target = Value::from_bool(input[bit / 8] & (1 << (bit % 8)) != 0);
    }
}
