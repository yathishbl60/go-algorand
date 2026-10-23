// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Value;
use p3_field::PrimeCharacteristicRing;

pub(super) fn auxiliary(bytes: [u8; 128]) -> [Value; 2624] {
    let mut auxiliary = [Value::ZERO; 2624];
    for limb in 0..64 {
        auxiliary[limb] =
            Value::from_u16(u16::from_le_bytes([bytes[2 * limb], bytes[2 * limb + 1]]));
    }
    for bit in 0..1024 {
        auxiliary[64 + bit] = Value::from_bool(input_bit(bytes, bit) != 0);
    }
    for i in 0..256 {
        let signed = input_bit(bytes, 4 * i) + input_bit(bytes, 4 * i + 1)
            - input_bit(bytes, 4 * i + 2)
            - input_bit(bytes, 4 * i + 3);
        auxiliary[1088 + 5 * i + (signed + 2) as usize] = Value::ONE;
        auxiliary[2368 + i] = Value::from_u16(signed.rem_euclid(3329) as u16);
    }
    auxiliary
}

fn input_bit(bytes: [u8; 128], bit: usize) -> i16 {
    ((bytes[bit / 8] >> (bit % 8)) & 1) as i16
}
