// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Gcm, hash::eval_hash, range::eval_ranges};
use crate::{
    Value,
    aes256_air::{eval_block, eval_key},
};
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

pub fn eval_gcm<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gcm<AB::Var>) {
    eval_key(builder, &value.key);
    eval_ranges(builder, value);
    for (index, block) in value.blocks.iter().enumerate() {
        eval_block(builder, &value.key, block);
        for byte in 0..16 {
            for bit in 0..8 {
                builder.assert_eq(
                    block.plaintext[byte][bit],
                    input_bit::<AB>(value, index, byte, bit),
                );
            }
        }
    }
    eval_ciphertext(builder, value);
    eval_hash(builder, value);
}

fn input_bit<AB: AirBuilder<F = Value>>(
    value: &Gcm<AB::Var>,
    index: usize,
    byte: usize,
    bit: usize,
) -> AB::Expr {
    if index == 0 {
        return AB::Expr::ZERO;
    }
    if byte < 12 {
        return value.nonce[byte][bit].into();
    }
    let counter = (index as u32).to_be_bytes();
    AB::Expr::from_bool(counter[byte - 12] & (1 << bit) != 0)
}

fn eval_ciphertext<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gcm<AB::Var>) {
    for byte in 0..super::PAYLOAD_BYTES {
        let stream = value.blocks[2 + byte / 16].state[14][byte % 16];
        for (bit, stream_bit) in stream.into_iter().enumerate() {
            let plaintext = AB::Expr::from(value.plaintext[byte][bit]);
            builder.assert_eq(
                value.ciphertext[byte][bit],
                plaintext.xor(&stream_bit.into()),
            );
        }
    }
}
