// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use p3_field::PrimeCharacteristicRing;

use crate::Value;
use crate::amounts::{Amounts, PublicValue};

impl Amounts<Value> {
    /// Constructs private range/carry columns after checked integer conservation.
    pub fn witness(input: [u64; 2], output: [u64; 2], public: PublicValue) -> Option<Self> {
        let incoming = input[0] as u128 + input[1] as u128 + public.debit as u128;
        let outgoing = output[0] as u128 + output[1] as u128 + public.credit as u128;
        if incoming != outgoing {
            return None;
        }
        let low = |v: u64| (v as u32) as i128;
        let difference = low(input[0]) + low(input[1]) + low(public.debit)
            - low(output[0])
            - low(output[1])
            - low(public.credit);
        let carry = (difference / (1i128 << 32) + 2) as u64;
        let values = [input[0], input[1], output[0], output[1]].map(bits);
        Some(Self {
            values,
            carry: bits(carry),
        })
    }
}

fn bits<const N: usize>(value: u64) -> [Value; N] {
    core::array::from_fn(|i| Value::from_bool(value & (1 << i) != 0))
}
