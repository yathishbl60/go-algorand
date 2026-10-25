// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{DisclosureInput, EPOCH, RHO, WIDTH};
use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(values: &mut [Value], input: &DisclosureInput<'_>) {
    let coefficients =
        core::array::from_fn::<_, 768, _>(|i| coefficient(input.envelope.public_key, i));
    for row in values.as_chunks_mut::<WIDTH>().0 {
        for (target, &value) in row[..RHO].iter_mut().zip(&coefficients) {
            *target = Value::from_u16(value);
        }
        bits(row, RHO, &input.envelope.public_key[1152..]);
        bits(row, EPOCH, &input.envelope.epoch.to_be_bytes());
    }
}

fn bits(row: &mut [Value], start: usize, bytes: &[u8]) {
    for bit in 0..bytes.len() * 8 {
        row[start + bit] = Value::from_bool(bytes[bit / 8] & (1 << (bit % 8)) != 0);
    }
}

fn coefficient(key: &[u8], index: usize) -> u16 {
    let start = index / 2 * 3;
    let packed = u32::from_le_bytes([key[start], key[start + 1], key[start + 2], 0]);
    ((packed >> (12 * (index % 2))) & 4095) as u16
}
