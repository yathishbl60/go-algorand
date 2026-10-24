// Copyright (C) 2026 Algorand Foundation Ltd.
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{INV_SPAN, INVERSE_ROW, WIDTH, WORK, prepared::Prepared};
use crate::{
    Value,
    mlkem_field::{Canonical, Q},
};
use p3_field::PrimeCharacteristicRing;

pub(super) fn write(values: &mut [Value], input: &Prepared<'_>) {
    for slot in 0..4 {
        let first = INVERSE_ROW + slot * INV_SPAN + 2048;
        for (index, row) in values.as_chunks_mut::<WIDTH>().0[first..first + 256]
            .iter_mut()
            .enumerate()
        {
            write_row(row, input, slot, index);
        }
    }
}

fn write_row(row: &mut [Value], input: &Prepared<'_>, slot: usize, index: usize) {
    let raw = input.raw[slot][index];
    let error = input.input.noise[3 + slot][index];
    let mu = input.input.message[index] * u16::from(slot == 3);
    let sum = (raw + error) % Q;
    for (offset, value) in [
        (0, raw),
        (25, error),
        (50, sum),
        (76, mu),
        (101, (sum + mu) % Q),
    ] {
        canonical(row, WORK + offset, value);
    }
    row[WORK + 75] = Value::from_bool(raw + error >= Q);
    row[WORK + 126] = Value::from_bool(sum + mu >= Q);
    for (index, &value) in input.raw[slot].iter().enumerate() {
        row[WORK + 512 + index] = Value::from_u16(value);
    }
}

fn canonical(row: &mut [Value], start: usize, input: u16) {
    let value = Canonical::witness(input).expect("canonical addition witness");
    row[start] = value.value;
    row[start + 1..start + 13].copy_from_slice(&value.bits);
    row[start + 13..start + 25].copy_from_slice(&value.gap);
}
