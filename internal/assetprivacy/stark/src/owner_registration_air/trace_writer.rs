// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{MESSAGE, ROWS, WIDTH, WORK, schedule};
use crate::{Value, serial_keccak as core, sha3_air::digest_values};
use p3_field::{Field, PrimeCharacteristicRing};

pub(super) fn build(
    message: &[u8; MESSAGE],
    secret: [u8; 32],
    group: [u8; 64],
) -> Option<Vec<Value>> {
    let mut values = Value::zero_vec(ROWS * WIDTH);
    let mut state = [0; 25];
    absorb(&mut state, message, 0);
    let count: u32 = secret.iter().map(|byte| byte.count_ones()).sum();
    let inverse = Value::from_u32(count).try_inverse()?;
    let group = digest_values(group);
    for (index, row) in values.as_chunks_mut::<WIDTH>().0.iter_mut().enumerate() {
        write_row(row, &state, inverse, &group);
        state = next_state(state, index, message)?;
    }
    Some(values)
}

fn next_state(mut state: [u64; 25], row: usize, message: &[u8; MESSAGE]) -> Option<[u64; 25]> {
    if let Some((phase, round)) = schedule::stage(row) {
        return match phase {
            0 => Some(core::theta(state)),
            1 => Some(core::rho_pi(state)),
            2 => core::chi_iota(state, round),
            _ => None,
        };
    }
    if row == 72 {
        absorb(&mut state, message, 72);
    }
    Some(state)
}

fn absorb(state: &mut [u64; 25], message: &[u8; MESSAGE], start: usize) {
    for offset in 0..72 {
        let index = start + offset;
        let byte = message.get(index).copied().unwrap_or(0)
            | (6 * u8::from(index == MESSAGE))
            | (0x80 * u8::from(index == 143));
        state[offset / 8] ^= u64::from(byte) << ((offset % 8) * 8);
    }
}

fn write_row(row: &mut [Value], state: &[u64; 25], inverse: Value, group: &[Value]) {
    row[0] = inverse;
    row[1..WORK].copy_from_slice(group);
    for index in 0..core::STATE_BITS {
        row[WORK + index] = Value::from_bool(state[index / 64] & (1 << (index % 64)) != 0);
    }
    write_parities(row, state);
}

fn write_parities(row: &mut [Value], state: &[u64; 25]) {
    for x in 0..5 {
        let parity = (0..5).fold(0, |acc, y| acc ^ state[y * 5 + x]);
        for z in 0..64 {
            row[WORK + core::STATE_BITS + x * 64 + z] = Value::from_bool(parity & (1 << z) != 0);
        }
    }
}
