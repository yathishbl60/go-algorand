// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{CARRY, POSITIVE, SOURCE_ACCOUNT, SOURCE_OWNER, WIDTH, WORK, prepared::Prepared};
use crate::{Value, sha3_air::digest_values};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(values: &mut [Value], input: &Prepared) {
    let owner = digest_values(input.owner);
    for row in values.as_chunks_mut::<WIDTH>().0 {
        for (index, bits) in input.amounts.values.iter().enumerate() {
            row[index * 64..index * 64 + 64].copy_from_slice(bits);
        }
        for bit in 0..256 {
            row[SOURCE_ACCOUNT + bit] =
                Value::from_bool(input.account[bit / 8] & (1 << (bit % 8)) != 0);
        }
        row[SOURCE_OWNER..CARRY].copy_from_slice(&owner);
        row[CARRY..POSITIVE].copy_from_slice(&input.amounts.carry);
        row[POSITIVE..WORK].copy_from_slice(&input.positive);
    }
}
