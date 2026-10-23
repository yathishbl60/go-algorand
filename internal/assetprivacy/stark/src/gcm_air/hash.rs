// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{Gcm, HASH_BLOCKS};
use crate::{
    Value,
    ghash_air::{eval_ghash, from_aes_bits, to_aes_bits},
};
use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

// BE64(64 * 8) || BE64(512 * 8): fixed full-block AAD and ciphertext lengths.
const LENGTHS: [u8; 16] = [0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0];

pub(super) fn eval_hash<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gcm<AB::Var>) {
    eval_ghash(builder, &value.ghash);
    let subkey = from_aes_bits(value.blocks[0].state[14]);
    for (bit, subkey_bit) in subkey.into_iter().enumerate() {
        builder.assert_eq(value.ghash.hash_subkey[bit], subkey_bit);
    }
    for block in 0..HASH_BLOCKS {
        for bit in 0..128 {
            builder.assert_eq(
                value.ghash.blocks[block][bit],
                block_bit::<AB>(value, block, bit),
            );
        }
    }
    eval_tag(builder, value);
}

fn block_bit<AB: AirBuilder<F = Value>>(
    value: &Gcm<AB::Var>,
    block: usize,
    bit: usize,
) -> AB::Expr {
    let (byte, aes_bit) = (bit / 8, 7 - bit % 8);
    if block < 4 {
        return value.associated[block * 16 + byte][aes_bit].into();
    }
    if block < 36 {
        return value.ciphertext[(block - 4) * 16 + byte][aes_bit].into();
    }
    AB::Expr::from_bool(LENGTHS[byte] & (1 << aes_bit) != 0)
}

fn eval_tag<AB: AirBuilder<F = Value>>(builder: &mut AB, value: &Gcm<AB::Var>) {
    let hash = to_aes_bits(value.ghash.output);
    for (byte, hash_byte) in hash.into_iter().enumerate() {
        for (bit, hash_bit) in hash_byte.into_iter().enumerate() {
            let mask: AB::Expr = value.blocks[1].state[14][byte][bit].into();
            builder.assert_eq(value.tag[byte][bit], mask.xor(&hash_bit.into()));
        }
    }
}
