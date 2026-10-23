// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integer constraints to compose into the complete note-spending relation.
//! These constraints alone do not establish ownership or authorize a transfer.

use p3_air::AirBuilder;
use p3_field::PrimeCharacteristicRing;

use crate::Value;

/// Two inputs followed by two outputs, with little-endian bit decompositions.
pub struct Amounts<T> {
    pub values: [[T; 64]; 4],
    pub carry: [T; 3],
}

/// Public conversion amounts must come from the bound transaction statement.
#[derive(Clone, Copy)]
pub struct PublicValue {
    pub debit: u64,
    pub credit: u64,
}

/// Constrains uint64 ranges and exact integer conservation without field wrap.
/// Callers must bind these same bits into note commitments and encryption.
pub fn eval<AB: AirBuilder<F = Value>>(
    builder: &mut AB,
    amounts: &Amounts<AB::Var>,
    public: PublicValue,
) {
    let limbs = amounts.values.map(|bits| range(builder, bits));
    let carry = signed_carry(builder, amounts.carry);
    let [low, high] = differences(limbs, public);
    builder.assert_zero(low - carry.clone() * Value::from_u64(1 << 32));
    builder.assert_zero(high + carry);
}

fn range<AB: AirBuilder<F = Value>>(builder: &mut AB, bits: [AB::Var; 64]) -> [AB::Expr; 2] {
    let mut limbs = [AB::Expr::ZERO, AB::Expr::ZERO];
    for (i, bit) in bits.into_iter().enumerate() {
        builder.assert_bool(bit);
        limbs[i / 32] += bit * Value::from_u64(1 << (i % 32));
    }
    limbs
}

fn signed_carry<AB: AirBuilder<F = Value>>(builder: &mut AB, bits: [AB::Var; 3]) -> AB::Expr {
    builder.assert_bools(bits);
    builder.assert_zero(bits[2] * (bits[0] + bits[1]));
    bits[0] + bits[1] * Value::TWO + bits[2] * Value::from_u8(4) - Value::TWO
}

fn differences<E: p3_field::Algebra<Value>>(limbs: [[E; 2]; 4], public: PublicValue) -> [E; 2] {
    let [[a0, a1], [b0, b1], [c0, c1], [d0, d1]] = limbs;
    let low = Value::from_u32(public.debit as u32) - Value::from_u32(public.credit as u32);
    let high = Value::from_u32((public.debit >> 32) as u32)
        - Value::from_u32((public.credit >> 32) as u32);
    [a0 + b0 - c0 - d0 + low, a1 + b1 - c1 - d1 + high]
}
